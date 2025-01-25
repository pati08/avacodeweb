# just task runner configuration for shuttle-template-yew

run-dev:
  trunk serve --open

build-release:
  trunk clean && trunk build --release
