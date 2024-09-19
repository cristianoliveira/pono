{ lib , rustPlatform , stdenv , darwin }:

rustPlatform.buildRustPackage {
  pname = "pono";
  version = "nightly-2024-09-16";

  ## build with local source
  src = ../.;

  cargoHash = "sha256-nDtkKPd0T5+kDc9hjd8rTAZEX2fc0uI3W4IUbf+ag1s=";

  buildInputs = lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreServices
  ];

  checkPhase = ''
    RUST_BACKTRACE=1 cargo test --test '*' -- --nocapture
  '';

  meta = with lib; {
    description = "pono - place and organize symlinks once";
    homepage = "https://github.com/cristianoliveira/pono";
    changelog = "https://github.com/cristianoliveira/pono/releases/tag/${src.rev}";
    license = licenses.mit;
    maintainers = with maintainers; [ cristianoliveira ];
  };
}

