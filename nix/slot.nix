{ lib , rustPlatform , fetchFromGitHub , stdenv , darwin }:

rustPlatform.buildRustPackage rec {
  pname = "slot";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "cristianoliveira";
    repo = "slot";
    rev = "v${version}";
    hash = "sha256-CYXMDYnE+c3aa8skkhHGAyRqXxHJ9UhSVP7iiNK2O+o=";
  };

  cargoHash = "sha256-sEfamaVCzHowNK/BidVJps/xXKKOPnV2yVo38GP9mb0=";

  checkPhase = ''
    cargo test
  '';

  buildInputs = lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.CoreServices
  ];

  meta = with lib; {
    description = "A symbolic link organizer with toml.";
    homepage = "https://github.com/cristianoliveira/slot";
    changelog = "https://github.com/cristianoliveira/slot/releases/tag/${src.rev}";
    license = licenses.mit;
    maintainers = with maintainers; [ cristianoliveira ];
  };
}

