# Study of a fragile pipeline 


Consider the following pipeline:

```
producer | transformer | consumer
```

How this pipeline should react to expected unrecoverable errors? How it should
react to bugs? This repo contains a study of several approaches

Further reading:

- https://medium.com/statuscode/pipeline-patterns-in-go-a37bb3a7e61d
- http://www.randomhacks.net/2019/03/08/should-rust-channels-panic-on-send/
- https://github.com/crossbeam-rs/crossbeam/issues/314
- https://github.com/golang/go/issues/14601#issuecomment-191421028
