{ lib , rustPlatform , stdenv , darwin }:

rustPlatform.buildRustPackage {
  pname = "pono";
  version = "local-20241004";

  ## build with local source
  src = ../.;

  cargoHash = "sha256-6mdy32QxH5eiif2yxYphIrN7esLaXRHffiBKj0kylZ4=";

  buildInputs = lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreServices
  ];

  checkPhase = ''
    RUST_BACKTRACE=1 cargo test
  '';

  meta = with lib; {
    description = "pono - place and organize symlinks once";
    homepage = "https://github.com/cristianoliveira/pono";
    changelog = "https://github.com/cristianoliveira/pono/releases/tag/${src.rev}";
    license = licenses.mit;
    maintainers = with maintainers; [ cristianoliveira ];
  };
}

