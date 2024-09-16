{ lib , rustPlatform , stdenv , darwin }:

rustPlatform.buildRustPackage {
  pname = "slot";
  version = "nightly-2024-09-16";

  ## build with local source
  src = ../.;

  cargoHash = "sha256-xdj6AOxYcfjq/EBPNzVdwlLRk1UL9MK1F7ISQWluWAQ=";

  buildInputs = lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreServices
  ];

  checkPhase = ''
    RUST_BACKTRACE=1 cargo test --test '*' -- --nocapture
  '';

  meta = with lib; {
    description = "A symbolic link organizer with toml.";
    homepage = "https://github.com/cristianoliveira/slot";
    changelog = "https://github.com/cristianoliveira/slot/releases";
    license = licenses.mit;
    maintainers = with maintainers; [ cristianoliveira ];
  };
}

