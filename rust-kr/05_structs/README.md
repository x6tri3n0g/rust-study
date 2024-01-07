# [05. 구조체로 연관된 데이터를 구조화하기](https://doc.rust-kr.org/ch05-00-structs.html)
*구조체(struct)* 는 여러 값을 묶고 이름을 지어서 의미있는 묶음을 정의하는 데에 사용합니다.
객체 지향 언어에서의 데이터 속성(attribute)와 비슷합니다. 앞서 배운 튜플과 구조체를 비교 대조하여 어떤 경우에 구조체로 데이터를 묶는 것이 더 좋은지 알아봅시다.  
- 구조체를 정의하고 생성하는 방법
- 구조체의 데이터와 연관된 동작을 표현하는 메서드, 연관 함수(associated functions) 정의 방법
구조체와 열거형(6장)은 **프로그램 도메인에 새로운 타입을 만들어서 러스트의 컴파일 시점 타입 검사 기능을 최대한 활용하기 위한 기본 구성요소**입니다.


## 구조체 정의 및 인스턴트화
  구조체는 여러 개의 연관된 값을 가질 수 없는 점(구성요소들은 각각 다른 타입이 될 수 있음)에서 `튜플 타입`과 비슷하다. 그렇지만 각각 구성요소에 이름을 붙일 수 있어 각 요소가 더 명확한 의미를 가질 수 있게 되고, 특정 요소에 접근할 때 순서에 의존할 필요도 없어 튜플보다 유연하게 사용이 가능하다.  

**구조체의 정의**
```rust
struct 해당 구조체에 지어줄 이름 {
  ...
  속성 이름(field): 속성 타입,
}
```
```rust
struct User {
  ...
  sign_in_count: u64,
}
```

**정의한 구조체 사용**
- 구조체의 각 필드에 대한 구체적인 값을 정하여 구조체의 *인스턴스* 를 생성해야 함
- 인스터스를 생성하기
  ```rust
  fn main() {
    let user1 = User {
      active: true,
      username: String::from("someusername123"),
      email: String::from("someone@example.com"),
      sign_in_count: 1,
    };
  }
  ```
  - 인스턴스 생성 시 필드의 순서는 동일하지 않아도 됨
- 구조체 내 특정 값은 점(.) 표기법으로 얻어올 수 있음
  ```rust
  user1.email = String::from("anotheremail@example.com");
  ```
- 인스턴스의 가변성
  - 가변성은 해당 인스턴스 전체가 가지고 있으며, 일부 필드만 가변으로 만들 수는 없음
- 다른 표현식과 마찬가지로 함수의 마지막 표현식에 구조체의 새 인스턴스를 생성하는 표현식으로 해당 인스턴스를 암묵적으로 반환할 수 있음
  - 변수명과 구조체 필드명이 동일한 경우 아래 `username`, `email`과 같이 사용할 수 있음
```rust
fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username,
    email,
    sign_in_count: 1,
  }
}
```

### 기존 인스턴스를 이용해 새 인스턴스를 만들 때 구조체 업데이트 문법 사용하기
*구조체 업데이트 문법(struct update syntax)* 을 사용하여 대부분의 값을 유지한 새로운 인스턴스를 생성하는데 사용할 수 있다.
```rust
fn main() {
  ...

  let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
  }
}
```
`..` 문법을 통해 나머지 필드를 주어진 인스턴스의 필드 값으로 설정한다.
```rust
fn main() {
  ...

  let user2 = User {
    email: String::from("another@example.com"),
    ..user1
  }
}
```
*JS의 Spread Operator와 비슷하지만 .이 두개라는 것
  
### 명명된 필드 없는 튜플 구조체를 사용하여 다른 타입 만들기
러스트는 *튜플 구조체(tuple structs)* 도 지원한다.  
`튜플 구조체`는 튜플 전체에 이름을 지어주거나 특정 튜플을 다른 튜플과 구분하고 싶은데, 그렇다고 각 필드명을 일일이 정해 일반적인 구조체 형태로 만들고 싶잊 않을 때 유용하다.  
  
`튜플 구조체`는 일반 구조체와 동일하게 정의할 수 있다.  
그렇지만 **튜플명 뒤에는 타입으로 이루어진 튜플로 이루어집니다.**
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0 , 0);
}
```
`black`, `origin`은 서로 다른 튜플 구조체로 만들어졌기 때문에 타입은 서로 다르다. 구성이 같더라도 각각의 구조체는 별도의 타입이다.
  
### 필드가 없는 유사 유닛 구조체
필드가 없는 구조체 정의도 가능하다. `튜플 타입`에서 다룬 유닛 타입 `()`와 비슷하게 동작하므로 *유사 유닛 구조체(unit-like structs)* 라 지칭한다. 유사 유닛 구조체는 어떤 타입에 대해 트레이트를 구현하고 싶지만 타입 내부에 어떤 데이터를 저장할 필요는 없는 경우 유용하다.  
*`트레이트`는 10장에서 다룬다.

```rust
struct AlwaysEqual;

fn main {
  let subject = AlwaysEqual;
}
```
이 구조체를 사용하게 되면 데이터는 필요하지 않다. 유사 유닛 구조체를 포함하여 임의의 타입에 대한 트레이트를 정의하고 구현하는 방법은 10장에서 다룬다.
  
### 구조체 데이터의 소유권
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
예제의 `User` 구조체는 의도적으로 `&str` 문자열 슬라이스 대신 구조체가 소유권을 갖는 `String` 타입을 사용한다. 왜냐하면 구조체 인스턴스가 유효한 동안 각 인스턴스 내의 모든 데이터가 유효하도록 만들기 위해서이다.  
참조자를 이용해 구조체가 소유권을 갖지 않는 데이터도 저장할 수는 있지만, 이는 10장에서 배울 *라이프타임(lifetime)* 을 활용해야 한다. 라이프타임을 사용하면 구조체가 존재하는 동안에 구조체 내 참조자가 가리키는 데이터의 유효함을 보장받을 수 있기 때문이다. 만약 라이프타임을 명시하지 않고 참조자를 저장하고자 하면 아래와 같은 문제가 발생한다.

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```

```
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` due to 2 previous errors
```
위 에러를 해결하여 구조체에 참조자를 저장하는 방법은 10장에서 알아봅니다. 지금 당장은 `&str` 대신 `String`을 사용하는 것으로 넘어가 봅니다. 

## 구조체를 사용한 예제 프로그램
```rust
fn main() {
    let width1 = 30;
    let height1 = 30;

    println!(
        "The area of the ractangle is {} square pixels",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```
```shell
Compiling ractangles v0.1.0 (/Study/rust-study/rust-kr/05_structs/ractangles)
    Finished dev [unoptimized + debuginfo] target(s) in 1.48s
     Running `target/debug/ractangles`
The area of the ractangle is 900 square pixels
```
위 코드는 각 치수 값으로 `area` 함수를 호출하여 사각형의 면적을 성공적으로 계산한다. 몇 가지 작업을 더하여 코드를 더 명료적하고 읽기 쉽게 만들 수 있다.  
`area` 함수의 시그니처를 보면 개선해야 할 점이 보인다.
```rust
fn area(width: u32, height: u32) -> u32 {
```
`area` 함수는 하나의 사각형의 면적을 계산한다. 두 개의 매개변수를 받고 있지만 두 값이 서로 연관되어 있다는 것을 명확하게 표현하는 부분은 없다. 두 값을 하나로 묶어버리면 코드의 가독성도 높아지고 관리하기도 쉬워진다. 이를 튜플로 해결해보자.

### 튜플로 리팩터링하기
```rust
fn main() {
    let rect1 = (30, 30);

    println!(
        "The area of the ractangle is {} square pixels",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```
튜플의 특정값을 접근하는 방법은 `{튜플명}.{인덱스}`와 같은 방법을 사용한다. 튜플을 사용함으로서 더 짜임새 있는 코드가 됐고, 인수도 단 하나만 넘기면 된다는 점에서 리팩토링을 해보았다. 하지만 튜플의 특성 때문에 값을 인덱스로 접근해야해서 계산식이 불명확졌다.  
다행히 넓이를 계산할 때는 각 요소의 너비, 높이 구분을 하지 않아도 된다. 하지만 사격형을 그리는 프로그램이라면 말이 두 요소에 대한 구분은 꼭 필요하다. 이를 해결하기 위해 다음에는 구조체를 통해 리팩터링 해보자.

### 구조체로 리팩터링하여 코드에 더 많은 의미를 담기
사각형에 대한 구조체를 정의하고 이를 사용하여 넓이를 구해보자.
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the ractangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```
`Rectangle`이라는 구조체를 정의하고 `width`, `height` 데이터를 정의한다. 그리고 main 함수 안에서 `rect1`에 인스턴스를 정의한다.  
main 함수 내에서 `area`를 호출 후에도 `Rectangle` 구조체를 사용할 수 있도록 참조자 타입으로 소유권을 빌려오기만 한다.(`&rect1`)  
참조자를 통해 `area`에 매개변수로 사용했기 때문에 필드 값을 이동시키지 않는다. 이제 `area` 함수의 시그니처는 이제 의미하는 바를 명확하게 알려준다. 이렇게 코드를 좀더 명료하게 만들었다.  

### 트레이트 파생으로 유용한 기능 추가하기
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```
이 코드는 동작하지 않는다. 
```shell
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```
`println!` 매크로에는 여러 출력 형식을 사용할 수 있다. 기본형인 `{}`로 지정할 땐 `Display`라는 최종 사용자를 위한 출력 형식을 사용한다. 여태 사용했던 기본 타입들은 `Display`가 기본적으로 구현되어 있었다. 예를 들어 `1`과 같은 기본 타입들은 사용자에게 보여줄 수 있는 형식이 딱 한가지 뿐이기 때문이다. 하지만 구조체는 그렇지 않다.(다양한 구조체의 형태가 존재할 수 있기 때문에) 따라서 러스트는 구조체의 `Display` 구현체를 제공하지 않는다.  
에러를 더 읽다 보면 다음과 같은 도움말을 찾을 수 있다.
```shell
  = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```
`{}` 대신 `{:?}`를 사용해 보라고 한다. `{:?}`는 `Debug`라는 출력 형식을 사용하고 싶다고 전달하는 것과 같다. `Debug`라는 트레이트는 최종 사용자가 아닌, 개발자에게 유용한 방식으로 출력하여 디버깅하는 동안 값을 볼 수 있게 해주는 트레이트이다.
```rust
println!("rect1 is {:?}", rect1);
```
위와 같이 수정하여 실행해보니 여전히 에러가 발행한다. 
```shell
error[E0277]: `Rectangle` doesn't implement `Debug`
...
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```
러스트는 디버깅 정보를 출력하는 기능을 자체적으로 가지고 있으나 우리가 만든 구조체에 해당 기능을 적용하려면 명시적인 동의가 필요하다.  
구조체 정의 바로 이전에 `#[derive(Debug)]` 외부 속성(outer attribute)을 작성해준다.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```
컴파일 시 안내와 같이 `{:#?}` 역시 사용할 수 있는데 이는 좀더 읽기 편한 방식으로 출력해준다.
```shell
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```
`Debug` 포맷을 사용하여 값을 출력하는 그 밖의 방법은 `dbg!` 매크로를 사용하는 것이 있다. 이는 표현식의 소유권을 가져와서 코드에서 `dbg!` 매크로를 호출한 파일 및 라인 번호를 결괏값과 함께 출력하고 다시 소유권을 반환한다.  
> Note: dbg! 매크로의 호출은 표준 에러 콘솔 스트림(stderr)에 출력을 하는데, 이는 표준 출력 콘솔 스트림(stdout)에 출력하는 println!과는 상반됩니다. stderr와 stdout에 대한 정보는 12장의 ‘표준 출력 대신 표준 에러로 에러 메시지 작성하기’절에서 더 이야기하겠습니다.

아래는 `rect1`의 전체 구조체 값뿐만 아니라 `width` 필드에 대입되는 값에 대한 정보를 얻기 위한 방법이다.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```
```shell
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```
line: 10의 디버깅 값을 보여준다. 이 처럼 `dbg!` 매크로는 현재 내가 작성한 코드가 어떤 일을 하고 있는지 알아볼 때 매우 유용하게 사용될 수 있다.
  
러스트에서는 이처럼 `Debug` 트레이트 말고도 `derive` 속성으로 직접 만든 타입에 유용한 동작을 추가할 수 있는 트레이트를 여럿 제공한다. 다양한 트레이트는 [원문의 부록 C](https://doc.rust-kr.org/appendix-03-derivable-traits.html)에서 확인이 가능하다. 이런 트레이트의 동작을 커스터마이징해서 구현하는 방법은 10장에서 배울 예정이다. 