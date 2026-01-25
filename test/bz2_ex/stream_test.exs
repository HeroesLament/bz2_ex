defmodule Bz2Ex.StreamTest do
  use ExUnit.Case, async: true

  describe "compression streaming" do
    test "compresses data in chunks" do
      {:ok, stream} = Bz2Ex.Stream.compress_init()
      {:ok, c1, stream} = Bz2Ex.Stream.compress(stream, "Hello, ")
      {:ok, c2, stream} = Bz2Ex.Stream.compress(stream, "World!")
      {:ok, final} = Bz2Ex.Stream.compress_finish(stream)

      compressed = IO.iodata_to_binary([c1, c2, final])
      {:ok, decompressed} = Bz2Ex.decompress(compressed)
      assert decompressed == "Hello, World!"
    end

    test "accepts options" do
      {:ok, stream} = Bz2Ex.Stream.compress_init(block_size: 5, work_factor: 50)
      {:ok, _, stream} = Bz2Ex.Stream.compress(stream, "test")
      {:ok, _} = Bz2Ex.Stream.compress_finish(stream)
    end

    test "handles large data" do
      {:ok, stream} = Bz2Ex.Stream.compress_init()
      original = :crypto.strong_rand_bytes(50_000)
      chunks = for <<chunk::binary-size(5000) <- original>>, do: chunk

      {compressed_chunks, stream} =
        Enum.map_reduce(chunks, stream, fn chunk, s ->
          {:ok, out, s} = Bz2Ex.Stream.compress(s, chunk)
          {out, s}
        end)

      {:ok, final} = Bz2Ex.Stream.compress_finish(stream)
      compressed = IO.iodata_to_binary(compressed_chunks ++ [final])
      {:ok, decompressed} = Bz2Ex.decompress(compressed)
      assert decompressed == original
    end
  end

  describe "decompression streaming" do
    test "decompresses in one chunk" do
      compressed = Bz2Ex.compress!("Hello, World!")
      {:ok, stream} = Bz2Ex.Stream.decompress_init()
      {:ok, data, :finished, _} = Bz2Ex.Stream.decompress(stream, compressed)
      assert data == "Hello, World!"
    end

    test "handles chunked input" do
      original = String.duplicate("test data ", 1000)
      compressed = Bz2Ex.compress!(original)

      chunk_size = div(byte_size(compressed), 3)
      c1 = binary_part(compressed, 0, chunk_size)
      c2 = binary_part(compressed, chunk_size, chunk_size)
      c3 = binary_part(compressed, chunk_size * 2, byte_size(compressed) - chunk_size * 2)

      {:ok, stream} = Bz2Ex.Stream.decompress_init()
      {:ok, d1, :ready, stream} = Bz2Ex.Stream.decompress(stream, c1)
      {:ok, d2, :ready, stream} = Bz2Ex.Stream.decompress(stream, c2)
      {:ok, d3, :finished, _} = Bz2Ex.Stream.decompress(stream, c3)

      assert IO.iodata_to_binary([d1, d2, d3]) == original
    end

    test "returns error for invalid data" do
      {:ok, stream} = Bz2Ex.Stream.decompress_init()
      {:error, :data_error_magic} = Bz2Ex.Stream.decompress(stream, <<1, 2, 3>>)
    end
  end

  describe "interoperability" do
    test "stream compress -> one-shot decompress" do
      {:ok, s} = Bz2Ex.Stream.compress_init()
      {:ok, c, s} = Bz2Ex.Stream.compress(s, "test")
      {:ok, f} = Bz2Ex.Stream.compress_finish(s)
      {:ok, d} = Bz2Ex.decompress(IO.iodata_to_binary([c, f]))
      assert d == "test"
    end

    test "one-shot compress -> stream decompress" do
      compressed = Bz2Ex.compress!("test")
      {:ok, s} = Bz2Ex.Stream.decompress_init()
      {:ok, d, :finished, _} = Bz2Ex.Stream.decompress(s, compressed)
      assert d == "test"
    end
  end
end
