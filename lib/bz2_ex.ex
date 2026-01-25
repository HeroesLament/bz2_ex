defmodule Bz2Ex do
  @moduledoc """
  A bzip2 compression library for Elixir.

  Provides bzip2 compression and decompression using a pure Rust implementation
  via `libbz2-rs-sys`. Offers both one-shot operations and streaming APIs.

  ## Examples

      {:ok, compressed} = Bz2Ex.compress("Hello, World!")
      {:ok, decompressed} = Bz2Ex.decompress(compressed)

  ## Options

  - `:block_size` - Integer 1-9. Block size is 100k × this value. Default: `9`.
  - `:work_factor` - Integer 0-250. Default: `0` (uses internal default of 30).
  - `:small` - Boolean. Use less memory but slower decompression. Default: `false`.
  """

  alias Bz2Ex.Native

  @type compress_opts :: [block_size: 1..9, work_factor: 0..250]
  @type decompress_opts :: [small: boolean()]
  @type error_reason ::
          :param_error
          | :mem_error
          | :data_error
          | :data_error_magic
          | :unexpected_eof
          | :outbuff_full
          | :config_error
          | :sequence_error
          | :unknown_error

  @doc """
  Compresses binary data using bzip2.

  ## Options

  - `:block_size` - Integer 1-9, default `9`
  - `:work_factor` - Integer 0-250, default `0`
  """
  @spec compress(binary(), compress_opts()) :: {:ok, binary()} | {:error, error_reason()}
  def compress(data, opts \\ []) when is_binary(data) do
    block_size = Keyword.get(opts, :block_size, 9)
    work_factor = Keyword.get(opts, :work_factor, 0)

    validate_block_size!(block_size)
    validate_work_factor!(work_factor)

    case Native.compress(data, block_size, work_factor) do
      {:ok, compressed} -> {:ok, compressed}
      {error_atom, _} -> {:error, error_atom}
    end
  end

  @doc "Compresses binary data, raising on error."
  @spec compress!(binary(), compress_opts()) :: binary()
  def compress!(data, opts \\ []) do
    case compress(data, opts) do
      {:ok, compressed} -> compressed
      {:error, reason} -> raise Bz2Ex.Error, reason: reason, operation: :compress
    end
  end

  @doc """
  Decompresses bzip2-compressed data.

  ## Options

  - `:small` - Boolean, default `false`
  """
  @spec decompress(binary(), decompress_opts()) :: {:ok, binary()} | {:error, error_reason()}
  def decompress(data, opts \\ []) when is_binary(data) do
    small = Keyword.get(opts, :small, false)

    case Native.decompress(data, small) do
      {:ok, decompressed} -> {:ok, decompressed}
      {error_atom, _} -> {:error, error_atom}
    end
  end

  @doc "Decompresses bzip2-compressed data, raising on error."
  @spec decompress!(binary(), decompress_opts()) :: binary()
  def decompress!(data, opts \\ []) do
    case decompress(data, opts) do
      {:ok, decompressed} -> decompressed
      {:error, reason} -> raise Bz2Ex.Error, reason: reason, operation: :decompress
    end
  end

  defp validate_block_size!(bs) when bs in 1..9, do: :ok
  defp validate_block_size!(bs), do: raise(ArgumentError, "block_size must be 1-9, got: #{inspect(bs)}")

  defp validate_work_factor!(wf) when wf in 0..250, do: :ok
  defp validate_work_factor!(wf), do: raise(ArgumentError, "work_factor must be 0-250, got: #{inspect(wf)}")
end
