defmodule Bz2ExTest do
  use ExUnit.Case, async: true

  describe "compress/2" do
    test "compresses binary data" do
      data = String.duplicate("hello world ", 1000)
      {:ok, compressed} = Bz2Ex.compress(data)
      assert is_binary(compressed)
      assert byte_size(compressed) < byte_size(data)
    end

    test "returns bzip2 magic header" do
      {:ok, compressed} = Bz2Ex.compress("test data")
      assert <<0x42, 0x5A, _::binary>> = compressed
    end

    test "accepts block_size option" do
      data = String.duplicate("x", 10_000)
      {:ok, c1} = Bz2Ex.compress(data, block_size: 1)
      {:ok, c9} = Bz2Ex.compress(data, block_size: 9)
      assert is_binary(c1) and is_binary(c9)
    end

    test "raises on invalid block_size" do
      assert_raise ArgumentError, fn -> Bz2Ex.compress("data", block_size: 0) end
      assert_raise ArgumentError, fn -> Bz2Ex.compress("data", block_size: 10) end
    end

    test "raises on invalid work_factor" do
      assert_raise ArgumentError, fn -> Bz2Ex.compress("data", work_factor: -1) end
      assert_raise ArgumentError, fn -> Bz2Ex.compress("data", work_factor: 251) end
    end

    test "handles empty binary" do
      {:ok, compressed} = Bz2Ex.compress(<<>>)
      assert is_binary(compressed)
    end
  end

  describe "decompress/2" do
    test "decompresses bzip2 data" do
      original = "Hello, World!"
      {:ok, compressed} = Bz2Ex.compress(original)
      {:ok, decompressed} = Bz2Ex.decompress(compressed)
      assert decompressed == original
    end

    test "round-trips large data" do
      original = :crypto.strong_rand_bytes(100_000)
      {:ok, compressed} = Bz2Ex.compress(original)
      {:ok, decompressed} = Bz2Ex.decompress(compressed)
      assert decompressed == original
    end

    test "returns error for invalid data" do
      {:error, :data_error_magic} = Bz2Ex.decompress(<<1, 2, 3, 4, 5>>)
    end

    test "accepts small option" do
      original = "test data"
      {:ok, compressed} = Bz2Ex.compress(original)
      {:ok, decompressed} = Bz2Ex.decompress(compressed, small: true)
      assert decompressed == original
    end
  end

  describe "bang variants" do
    test "compress! returns data directly" do
      assert is_binary(Bz2Ex.compress!("hello"))
    end

    test "decompress! returns data directly" do
      compressed = Bz2Ex.compress!("hello")
      assert Bz2Ex.decompress!(compressed) == "hello"
    end

    test "decompress! raises on invalid data" do
      assert_raise Bz2Ex.Error, fn -> Bz2Ex.decompress!(<<1, 2, 3>>) end
    end
  end
end
