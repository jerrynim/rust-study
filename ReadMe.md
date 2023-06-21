### cargo 명령어

교제: https://rinthel.github.io/rust-lang-book-ko/foreword.html

```
$ cargo run
$ cargo build
$ cargo check
$ cargo build --release
왜 여러분이 실행파일을 원치 않게 될까요? 종종 cargo check가 cargo build에 비해 훨씬 빠른데, 그 이유는 이 커맨드가 실행파일을 생성하는 단계를 생략하기 때문입니다. 만일 여러분이 코드를 작성하는 동안 계속적으로 여러분의 작업물을 검사하는 중이라면, cargo check를 이용하는 것이 그 과정의 속도를 높여줄 것입니다! 그런 이유로, 많은 러스트인들이 자신들의 프로그램을 작성하면서 이것이 컴파일 되는지 확인하기 위해 주기적으로 cargo check을 실행합니다. 그런 다음 실행파일을 사용할 준비가 되었을 때 cargo build를 실행합니다.

$ cargo update
$ cargo doc --open 명령어로써 로컬에서 여러분의 모든 의존 패키지들이 제공하는 문서들을 빌드해서 브라우저에 표시해 줍니다.
```
