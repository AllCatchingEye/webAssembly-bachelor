# Build lib and command component
(cd ./add_run && cargo component build --release)
(cd ./lib-component && ./lib-component/build.sh)

RUNNER_COMPONENT="./add_run/target/wasm32-wasi/release/add_run.wasm"
ADD_COMPONENT="./lib-component/add-component.wasm"
APP="./add-app.wasm"

# Compose lib and command component
wasm-tools compose $RUNNER_COMPONENT -d $ADD_COMPONENT -o $APP
