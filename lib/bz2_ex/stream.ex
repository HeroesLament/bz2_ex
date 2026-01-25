defmodule Bz2Ex.Stream do
  @moduledoc """
  Streaming bzip2 compression and decompression.

  ## Compression

      {:ok, stream} = Bz2Ex.Stream.compress_init()
      {:ok, chunk1, stream} = Bz2Ex.Stream.compress(stream, "Hello, ")
      {:ok, chunk2, stream} = Bz2Ex.Stream.compress(stream, "World!")
      {:ok, final} = Bz2Ex.Stream.compress_finish(stream)
      compressed = IO.iodata_to_binary([chunk1, chunk2, final])

  ## Decompression

      {:ok, stream} = Bz2Ex.Stream.decompress_init()
      {:ok, data, :finished, _stream} = Bz2Ex.Stream.decompress(stream, compressed)
  """

  alias Bz2Ex.Native

  @opaque compress_stream :: reference()
  @opaque decompress_stream :: reference()
  @type compress_opts :: [block_size: 1..9, work_factor: 0..250]
  @type decompress_opts :: [small: boolean()]
  @type decompress_status :: :ready | :finished

  @doc "Initialize a compression stream."
  @spec compress_init(compress_opts()) :: {:ok, compress_stream()} | {:error, Bz2Ex.error_reason()}
  def compress_init(opts \\ []) do
    block_size = Keyword.get(opts, :block_size, 9)
    work_factor = Keyword.get(opts, :work_factor, 0)
    validate_block_size!(block_size)
    validate_work_factor!(work_factor)
    Native.compress_stream_init(block_size, work_factor)
  end

  @doc "Feed data into a compression stream."
  @spec compress(compress_stream(), binary()) ::
          {:ok, binary(), compress_stream()} | {:error, Bz2Ex.error_reason()}
  def compress(stream, data) when is_binary(data) do
    case Native.compress_stream_deflate(stream, data) do
      {:ok, chunk} -> {:ok, chunk, stream}
      {error_atom, _} -> {:error, error_atom}
    end
  end

  @doc "Finish a compression stream and get remaining data."
  @spec compress_finish(compress_stream()) :: {:ok, binary()} | {:error, Bz2Ex.error_reason()}
  def compress_finish(stream) do
    case Native.compress_stream_finish(stream) do
      {:ok, chunk} -> {:ok, chunk}
      {error_atom, _} -> {:error, error_atom}
    end
  end

  @doc "Initialize a decompression stream."
  @spec decompress_init(decompress_opts()) ::
          {:ok, decompress_stream()} | {:error, Bz2Ex.error_reason()}
  def decompress_init(opts \\ []) do
    small = Keyword.get(opts, :small, false)
    Native.decompress_stream_init(small)
  end

  @doc "Feed compressed data into a decompression stream."
  @spec decompress(decompress_stream(), binary()) ::
          {:ok, binary(), decompress_status(), decompress_stream()}
          | {:error, Bz2Ex.error_reason()}
  def decompress(stream, data) when is_binary(data) do
    case Native.decompress_stream_inflate(stream, data) do
      {:ok, chunk, status} when status in [:ready, :finished] ->
        {:ok, chunk, status, stream}

      {error_atom, _, _} ->
        {:error, error_atom}
    end
  end

  defp validate_block_size!(bs) when bs in 1..9, do: :ok
  defp validate_block_size!(bs), do: raise(ArgumentError, "block_size must be 1-9, got: #{inspect(bs)}")

  defp validate_work_factor!(wf) when wf in 0..250, do: :ok
  defp validate_work_factor!(wf), do: raise(ArgumentError, "work_factor must be 0-250, got: #{inspect(wf)}")
end
