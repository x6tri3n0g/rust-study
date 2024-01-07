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