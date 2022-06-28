cd ./agent && cargo rustc --release -- --crate-type=cdylib
rm -rf ./plugin-unity/IC/agent.dylib
cd .. && cp ./agent/target/release/libagent.dylib ./plugin-unity/Assets/IC/agent.dylib