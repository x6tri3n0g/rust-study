# 열거형과 패턴 매칭  

> 이 장에서는 rust의 열거형이 데이터와 함께 의미를 담을 수 있는지 알아봅니다.

- 열거형(enumerations; enums)
  - 하나의 타입이 가질 수 있는 배리언트(variant) 들을 열거함으로써 타입을 정의할 수 있도록 함
- `Option`: 값이 어떤 것일 수도 있고 아무것도 아닐수도 있음을 표현하는 열거형
- `match`: 패턴 매칭을 통해 열거형의 값에 따라 다른 코드를 쉽게 실행할 수 있음
- `if let`: 열거형을 편하고 간결하게 다루기 위한 관용 표현 구문

## 열거형 정의하기
- 구조체는 서로 연관된 필드 및 데이터를 묶는 방법을 제공함
- 열거형은 어떤 값이 여러 개의 가능한 값의 집합 중 하나라는 것을 나타내는 방법
  
  
예시로 IP 주소를 열거형으로 표시하는 방법을 만들어 봅니다.(IP 주소 표준은 IPv4, IPv6 두 종류)
```rust
enum IpAddrKind {
  V4,
  V6,
}
```
- `IpAddrKind`라는 열거형을 정의하면서 포함할 수 있는 IP 주소인 `V4`과 `V6`를 나열함
- 이 개념을 코드에 표현한 것을 *열거형의 배리언트* 라고 함
  
### 열거형 값
위 정의한 enum의 두개의 배리언트에 대한 인스턴스를 만들 수 있습니다:
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
- 열거형 정의 시 식별자로 네임스페이스가 만들어져서, 각 배리언트 앞에 이중 콜론(`::`)을 붙여야 함
- `IpAddrKind::V4`, `IpAddrKind::V6`
  
  
`IpAddrKind` 타입을 인수로 받는 함수를 정의/호출하기
```rust
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```
  
열거형을 사용하면 가질 수 있는 이점
- 현재 실제 IP 주소 데이터를 저장할 방법이 없고 어떤 종류인지만 알 수 있음
```rust
// struct를 사용해서 IP 주소의 데이터와 IpAddrKind 배리언트 저장하기
enum IpAddrKind {
  V4,
  V6,
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

let home = IpAddr {
  kind: IpAddrKind::V4,
  address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1"),
};
```

각 열거형 배리언트에 데이터를 직접 넣는 방식을 사용해서 열거형을 구조체의 일부로 사용하는 방식보다 더 간결하게 동일한 개념으로 표현이 가능하다.
```rust
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```
- 열거형의 각 배리언트에 직접 데이터를 붙임으로써, 구조체를 사용할 필요가 없어짐
- 열거형의 동작에 대한 다른 세부 사항을 사항을 살펴보기 쉬움
  
  
각 배리언트는 다른 타입과 다른 크기의 연관된 데이터를 가질 수 있다. V4 IP 주소는 항상 0 ~ 255 사이의 숫자 4개로 된 구성 요소를 갖게 된다. V6 주소는 하나의 `String` 값으로 표현되길 원한다면 아래와 같이 표현할수 있다.
```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

  
실제로 IP 주소와 그 종류를 저장하는 표준 라이브러리에서는 아래와 같이 정의한다.
```rust
struct Ipv4Addr {
    // --생략--
}

struct Ipv6Addr {
    // --생략--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
- 코드에서도 알 수 있듯, 배리언트에는 어떤 종류의 데이터라도 넣을 수 있음
  - 문자열, 숫자 타입, 구조체 등은 물론, 다른 열거형 마저도 포함 가능함
  
열거형의 다른 예제를 살펴본다. 이 예제에서는 각 배리언트에 다양한 종류의 타입들이 포함되어 있다.
```rust
// Message 열거형의 각 배리언트가 다른 타입과 다른 크기의 값을 저장한다.
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
```

배리언트로 열거형을 정의하는 것은 다른 종류의 구조체들을 정의하는 것과 비슷하다. 열거형과 다른 점은 `struct` 키워드를 사용하지 않는다는 것과 모든 배리언트가 `Message` 타입으로 묶인다는 것이다. 아래 구조체들은 이전 열거형의 배리언트가 갖는 것과 동일한 데이터를 가질 수 있다.
```rust
struct QuitMessage; // 유닛 구조체
struct MoveMessage {
  x: i32,
  y: i32,
}
struct WriteMessage(String);  // 튜플 구조체
struct ChanageColorMessage(i32, i32, i32);  // 튜플 구조체
```
각기 다른 타입을 갖는 여러 개의 구조체를 사용한다면, 이 메시지 중 어떤 한 가지를 인수로 받는 함수를 정의하기 힘들 것이다.  
열거형과 구조체는 한 가지 더 유사한 점이 있다. 구조체에 `impl`을 사용해서 메서드를 정의한 것처럼, 열거형에도 정의할 수 있다. 아래에 `Message` 열거형에 정의한 `call`이라는 메서드가 있다.
```rust
impl Message {
  fn call(&self) {
    // 메서드 본문
  }
}

let m = Message::Write(String::from("hello"));
m.call();
```
**열거형(enum)도 구조체(struct)와 동일하게 메서드를 `impl`를 통해서 정의한다.**

### `Option` 열거형이 널 값보다 좋은 점들
`Option` 타입은 값이 있거나 없을 수 있는 아주 흔한 상황을 나타낸다.  

예를 들어 빈 리스트의 첫 번째 아이템을 요청한다면 값을 얻을 수 없다. 이 개념을 타입 시스템으로 표현한다는 것은 처리해야하는 모든 경우를 처리했는지 컴파일러가 확인할 수 있다는 의미이다; 이러한 기능은 다른 프로그래밍 언어에서 매우 흔하게 발생하는 버그를 방지해준다.  
  
프로그래밍 언어 디자인은 가끔 어떤 기능들이 포함되었는지의 관점에서 생각되기도 하지만, 어떤 기능을 포함하지 않을 것이냐도 중요하다. 러스트는 다른 언어들에서 흔하게 볼 수 있는 널(null) 개념이 없다. `Null`은 값이 없음을 표현하는 하나의 값이다. `Null`이 존재하는 언어에서, 변수의 상태는 둘 중 하나이다.(`Null`인 경우와 아닌 경우)  

*`Null`을 고안한 토니 호어(Tony Hoare)의 '널 참조: 10억 달러짜리 실수(Null References: The Billion Dollar Mistake)'
> 저는 그걸 10억 달러짜리 실수라고 부릅니다. 저는 그 당시 객체 지향 언어에서 참조를 위한 첫 포괄적인 타입 시스템을 디자인하고 있었습니다. 제 목표는 컴파일러에 의해 자동으로 수행되는 체크를 통해 모든 참조자의 사용이 절대로 안전함을 보장하는 것이었습니다. 하지만 구현이 무척 간단하다는 단순한 이유로 널 참조를 넣고 싶은 유혹을 참을 수 없었습니다. 이는 수없이 많은 에러와 취약점, 시스템 종료를 유발했고, 아마도 지난 40년간 10억 달러 수준의 고통과 손실을 초래해왔습니다.
  
Null 값으로 발생하는 문제는 Null 값을 Null 값이 아닌 것처럼 사용하려고 할 때 여러 종류의 에러가 발생할 수 있다는 것이다. Null이나 Null이 아닌 속성은 어디에나 있을 수 있고, 너무나도 쉽게 이런 종류의 에러를 만들어 낸다.(그렇지만 어떤 이유로 인해 유효하지 않거나, 존재하지 않는 하나의 값이라는 Null이 표현하고자 하는 개념은 여전히 유용하다.)  
  
러스트에서 Null은 실제 개념이 있기보다 특정 구현에 있다. 즉 Null은 없지만 값의 존재 혹은 부재의 경험을 표현할 수 있는 열거형이 있습니다. 그것이 바로 `Option<T>`이다.(표준 라이브러리에 정의됨)  
```rust
enum Option<T> {
  None,
  Some(T),
}
```
`Option<T>` 열거형은 유용하기 때문에 러스트에서 기본으로 임포트하는 목록인 프렐루드에도 포함되어 있다. 
*[Rust Prelude](https://doc.rust-lang.org/std/prelude/index.html)
> Rust comes with a variety of things in its standard library. However, if you had to manually import every single thing that you used, it would be very verbose. But importing a lot of things that a program never uses isn’t good either. A balance needs to be struck. The prelude is the list of things that Rust automatically imports into every Rust program. It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.
  
```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```
- `Some`과 `None`은 prelude에 포함되기 때문에 `Option::`을 붙이지 않아도 됨
- `Some` 배리언트가 어떤 타입의 데이터라도 담을 수 있음
  - 러스트는 `Some` 배리언트 내에 어떤 값을 명시했기 때문에 이 타입들을 추론할 수 있음
- `None`은 얻는 값이 유효하지 않음을 의미
  - Null과 같은 의미
  
`Option<T>`가 왜 Null보다 나은가? `Option<T>`와 `T`(`T`는 어떤 타입이던 될 수 있음)이 다른 타입이기 때문에, 컴파일러는 `Option<T>`값을 명백하게 유효한 값처럼 사용하지 못하도록 한다. 예를 들면 sum은 컴파일되지 않는다.
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```
```shell
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <&'a i8 as Add<i8>>
            <&i8 as Add<&i8>>
            <i8 as Add<&i8>>
            <i8 as Add>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enums` due to previous error
```
- 이 메시지는 러스트가 `Option<i8>`와 `i8`은 서로 다른 타입이기 때문에 어떻게 더해야 하는지 모른다는 것을 의미한다.   
  
Null이 아닌 값을 갖는다는 가정을 놓치는 경우에 대한 위험 요소가 제거되며느 코드에 더 확신을 갖을 수 있다. Null일 수 있는 값을 사용하기 위해서는 명시적으로 값의 타입을 `Option<T>`로 만들어 줘야 한다. 그 다음엔 값을 사용할 때 명시적으로 Null인 경우를 처리해야 한다. 그러므로써 값의 타입이 `Option<T>`가 아닌 곳은 값이 Null이 아니라고 안전하게 *가정할 수 있다.*  
  
`Option<T>` 값을 사용하기 위해서는 각 배리언트를 처리할 코드가 필요할 것이다. `Some<T>` 값일 때만 실행돼서 내부의 `T`값을 사용하는 코드도 필요하고 `None` 값일 때만 실행될, `T` 값을 쓸 수 없는 코드도 필요하다.