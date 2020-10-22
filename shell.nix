with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "env-api";
  nativeBuildInputs = [
    openssl
    pkgconfig
    zlib
  ];
  buildInputs = [
    protobuf
    grpcurl

    wasmtime
    openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;

  PROTOC="${protobuf.outPath}/bin/protoc";
  GOOGLE_PROTOBUF="${protobuf.outPath}/include/";

  shellHook =
  ''
    export HISTFILE=/tmp/runnable_bash_history
    export PATH=~/.local/share/cargo/bin:$PATH
    export PS1='{env-api} [\u@\h \W]\$ ';
  '';
}

