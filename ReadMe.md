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

Shadowing
데이터 타입의 변경이 가능하고 불변성에 유리

```
  let x = x + 1;

  let x = x * 2;
```

### 데이터 타입

스칼라는 하나의 값으로 표현되는 타입입니다. Rust는 정수형, 부동소수점 숫자, boolean, 그리고 문자, 네 가지 스칼라 타입을 보유하고 있습니다.
정수형
8-bit i8 u8
16-bit i16 u16
32-bit i32 u32
64-bit i64 u64
arch isize usize
isize와 usize타입은 당신의 프로그램이 동작하는 컴퓨터 환경이 64-bits인지 아닌지에 따라 결정됩니다. 64-bit 아키텍처이면 64bit를, 32-bit 아키텍처이면 32bit를 갖게 됩니다.isize나 usize는 주로 일부 콜렉션 타입의 색인에 사용됩니다.

부동 소수점 타입
f32, f64

문자 타입
스트링이 큰따옴표를 쓰는 것에 반하여 char 타입은 작은따옴표로 쓰는 점을 주목하세요:

복합 타입들
복합 타입들은 다른 타입의 다양한 값들을 하나의 타입으로 묶을 수 있습니다. Rust는 두 개의 기본 타입들을 갖고 있습니다: 튜플과 배열.

튜플

```
n main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```
