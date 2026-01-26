defmodule Bz2Ex.MixProject do
  use Mix.Project

  @version "0.1.1"
  @source_url "https://github.com/HeroesLament/bz2-ex"

  def project do
    [
      app: :bz2_ex,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: description(),
      package: package(),
      docs: docs(),
      name: "Bz2Ex",
      source_url: @source_url
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      # NIF compilation
      {:rustler, "~> 0.37", optional: true},
      {:rustler_precompiled, "~> 0.8"},

      # Development & testing
      {:ex_doc, "~> 0.31", only: :dev, runtime: false},
      {:dialyxir, "~> 1.4", only: [:dev, :test], runtime: false}
    ]
  end

  defp description do
    """
    A bzip2 compression library for Elixir, powered by a pure Rust implementation
    via libbz2-rs-sys. Provides both one-shot and streaming compression/decompression.
    """
  end

  defp package do
    [
      name: "bz2_ex",
      licenses: ["MIT"],
      links: %{
        "GitHub" => @source_url
      },
      files: [
        "lib",
        "native/bz2_ex/.cargo",
        "native/bz2_ex/Cargo.toml",
        "native/bz2_ex/Cargo.lock",
        "native/bz2_ex/src",
        "checksum-Elixir.Bz2Ex.Native.exs",
        "mix.exs",
        "README.md",
        "LICENSE"
      ]
    ]
  end

  defp docs do
    [
      main: "Bz2Ex",
      extras: ["README.md"],
      source_ref: "v#{@version}",
      source_url: @source_url
    ]
  end
end
