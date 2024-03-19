# nom guide 

- workflow
```shell
just work
```

- To run a specific test
```shell
cargo test --package nom-guide --lib -- chapter01::tests::test_do_nothing_parser --exact --nocapture 
```