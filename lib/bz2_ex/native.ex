defmodule Bz2Ex.Native do
  @moduledoc false

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :bz2_ex,
    crate: "bz2_ex",
    base_url: "https://github.com/YOURNAME/bz2-ex/releases/download/v#{version}",
    force_build: System.get_env("BZ2_EX_BUILD") in ["1", "true"],
    targets: ~w(
      aarch64-apple-darwin
      aarch64-unknown-linux-gnu
      aarch64-unknown-linux-musl
      x86_64-apple-darwin
      x86_64-unknown-linux-gnu
      x86_64-unknown-linux-musl
      x86_64-pc-windows-msvc
      x86_64-pc-windows-gnu
    ),
    version: version

  def compress(_input, _block_size, _work_factor), do: :erlang.nif_error(:nif_not_loaded)
  def decompress(_input, _small), do: :erlang.nif_error(:nif_not_loaded)
  def compress_stream_init(_block_size, _work_factor), do: :erlang.nif_error(:nif_not_loaded)
  def compress_stream_deflate(_stream, _input), do: :erlang.nif_error(:nif_not_loaded)
  def compress_stream_finish(_stream), do: :erlang.nif_error(:nif_not_loaded)
  def decompress_stream_init(_small), do: :erlang.nif_error(:nif_not_loaded)
  def decompress_stream_inflate(_stream, _input), do: :erlang.nif_error(:nif_not_loaded)
end
