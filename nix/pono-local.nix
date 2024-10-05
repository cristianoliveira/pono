{ lib , rustPlatform , stdenv , darwin }:

rustPlatform.buildRustPackage {
  pname = "pono";
  version = "local-20241005";

  ## build with local source
  src = ../.;

  cargoHash = "sha256-Ge/vlWeWxmjLatnlI+tQMZA9BscCYro7OQjeG4nupEU=";

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

