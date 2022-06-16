cd ./agent && cargo rustc --release -- --crate-type=cdylib
cd .. && cp ./agent/target/release/libagent.dylib ./exp-cs/agent.dylib
cd ./exp-cs && dotnet run