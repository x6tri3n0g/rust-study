# 02. 추리게임
해당 장에서는 실습 프로젝트를 통해서 `let`, `match`, 메서드, 연관 함수(associated functions), 외부 크레이트(external crates) 등의 기초적인 활용 방법을 배워본다.

**요구사항**
- 1~100 사이에 임의의 정수를 생성
- 플레이어가 추리한 정수를 입력
- 프로그램은 입력받은 추릿값이 정답보다 높거나 낮음을 알려줌
- 추리값이 정답이라면 축하 메시지를 보여주고 종료

## 프로젝트 준비하기
```shell
$ cargo new guessing_game
$ cd guessing_game
```

## 추리값을 처리하기
프로그램의 첫 부분에서는 사용자 입력을 요청하고 입력값의 처리 후 입력값이 기대하던 형식인지 검증합니다.  
첫 시작으로 플레이어가 추리한 값을 입력받을 수 있게 할 것입니다.

```rust
// src/main.rs
// 사용자가 추리하여 입력한 값을 그대로 출력
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```
- `use std::io;` : 사용자 입력을 받고 결괏값을 표시하기 위한 `io` 입출력 라이브러리 스코프, `io` 라이브러리는 `std`라고 불리는 표준 라이브러리에 있음
- 기본적으로 러스트는 모든 프로그램의 스코프로 가져오는 표준 라이브러리에 정의된 아이템 집합을 가지고 있습니다.
  - 이 집합을 프렐루드(*prelude*)라고 부름, 이와 관련된 것은 [표준 라이브러리 문서](https://doc.rust-lang.org/std/index.html)에서 찾아볼 수 있음
  - 만약 원하는 타입이 프렐루드에 없다면 `use`문을 활용하여 명시적으로 그 타입을 가져와야함
  - `std::io`는 사용자의 입력을 받는 것을 포함하여 `io`와 관련된 기능들을 제공함
- `fn main() {`
  - `fn` 키워드는 새로운 함수를 선언함을 의미
  - `()`는 매개변수가 없을 나타냄
  - `{`는 함수 본문의 시작을 의미
- `println!("Guess the number!");` : 문자열을 화면에 출력하는 매크로
  - 프롬프트에 출력됨

### 변수에 값 저장하기
```rust
let mut guess = String::new();
```
사용자의 입력값을 저장할 변수(*variable*)를 생성합니다.

변수를 만드는데는 `let` 키워드를 사용합니다.
```rust
let apples = 5;
```
- `apples`라는 변수를 만들고 5라는 값을 대입합니다.
- 러스트에서 변수는 기본적으로 불변(immutable)입니다.
  - 변수에 어떤 값을 넣으면 그 값이 안 바뀔 것이라는 뜻
  - 3장의 [변수와 가변성](https://rust-kr.github.io/doc.rust-kr.org/ch03-01-variables-and-mutability.html#variables-and-mutability)절에서 추가로 논의할 예정
  - 변수의 값을 가변(mutable), 즉 변경 가능하도록 하려면 변수명 앞에 `mut`를 추가
    - `let mut apples = 5;` // mutable variable

```rust
let mut guess = String::new();
```
는 **가변 변수**이며 변수명은 `guess`입니다. 그리고 여기에는 `String:new()`의 결괏값인 새로운 `String` 인스턴스가 생성됩니다.
`String`은 표준 라이브러리에서 제공하는 확장 가능한(growable) UTF-8 인코딩의 문자열 타입입니다.  
  
`::new`에 있는 `::`는 `new`가 `String` 타입의 연관 함수(associated function)임을 나타냅니다. 
- **연관 함수**란 어떤 타입에 구현된 함수라는 의미    

이 `new` 함수는 비어있는 새 문자열을 생성합니다. `new`는 어떤 새로운 값을 만드는 함수 이름으로 흔히 사용되는 이름이기 때문에, 여러 타입에서 `new` 함수를 찾아볼 수 있을 겁니다.
요약하자면 새로운 빈 `String` 인스턴스를 묶어넣은 가변 변수를 생성합니다.

### 사용자 입력받기
```rust
io::stdin()
    .read_line(&mut guess)
```
프로그램 시작점에서 `use std::io`를 통해 `io` 라이브러리를 가져오지 않더라도, 함수 호출 시 `std::io::stdin` 처럼 작성하는 것으로 이 함수를 사용할 수도 있습니다.
`stdin` 함수는 터미널의 표준 입력 핸들(handle)을 나타내는 타입인 `std::io::Stdin`의 인스턴스를 돌려줍니다.

`.read_line(&mut guess)`는 사용자로부터 입력받기 위해 표준 입력 핸들에서 `read_line` 메서드를 호출하는 것입니다.  
`&mut guess`를 `read_line`의 인수로 전달하여 사용자 입력이 어떤 문자열에 저장될 것인지 알려줍니다.

`&`는 코드의 여러 부분에서 데이터를 여러번 메모리로 복사하지 않고 접근하기 위한 방법을 제공하는 참조자(*reference*)임을 나타냅니다.
 - 참조는 복잡한 기능이고, 러스트의 큰 이점 중 하나가 바로 참조자를 사용할 때의 안전성과 편의성입니다.
 - 참조자는 변수처럼 기본적으로 불변임

### Result 타입으로 잠재적 실패 다루기
```rust
.expect("Failed to read line");
```
입력에 실패하는 경우 해당 명령문이 실행됩니다.

`Result`는 *enum*이라고도 일컫는 **열거형(enumeration)**인데, 여러 개의 가능한 상태 중 하나의 값이 될 수 있는 타입입니다. 이런 가능한 상태 값을 배리언트(*variant*)라고 부릅니다.
이 `Result` 타입의 목적은 에러 처리용 정보를 담아내기 위한 것입니다.
- `Result`의 배리언트는 `Ok`와 `Err`입니다.
  - `Ok`는 처리가 성공했음을 나타내며 내부에 성공적으로 생성된 결과를 가지고 있음
  - `Err`는 처리가 실패했음을 나타내고 그 이유에 대한 정보를 가지고 있음

만약 `expect()`를 호출하지 않는다면 컴파일은 되지만 경고가 나타납니다.  
러스트는 `read_line`가 돌려주는 `Result` 값을 사용하지 않았음을 경고하며 일어날 수 있는 에러를 처리하지 않았음을 알려줍니다.
[9장](https://rust-kr.github.io/doc.rust-kr.org/ch09-02-recoverable-errors-with-result.html)에서 에러를 복구하는 방법에 대해 배우게 됩니다.

### println! 자리표자를 이용한 값 출력하기
```rust
println!("You guessed: {guess}");
```
이 라인은 사용자가 입력한 값을 담고 있는 문자열을 출력합니다. `{}`는 자리표시자(placeholder)입니다. `{}`를 어떤 위치에 값을 자리할수 있도록 합니다.

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2); // "x = 5 and y + 2 = 12"를 출력
```

### 첫번째 부분 테스트하기
```shell
$ cargo run
Guess the number!
Please input your guess.
6
You guessed: 6
```

## 비밀번호 생성하기
1~100 사이의 임의의 수를 생성하기 위해서 [rand 크레이에트(crate)](https://crates.io/crates/rand)를 사용해봅시다.

### 크레이트를 사용하여 더 많은 기능 가져오기
- 크레이트는 러스트 코드 파일들의 모음
- 우리가 만들고 잇는 프로젝트는 실행이 가능한 *바이너리 크레이트(binary crate)*
- `rand` 크레이트는 자체적으로 실행될 수는 없고 다른 프로그램에서 사용되기 위한 용도인 *라이브러리 크레이트(library crate)*

Cargo.toml에 rand 크레이트를 추가합니다.  
```toml
[dependencies]
rand = "0.8.5"
```

`[dependencies]`에는 프로젝트가 의존하고 있는 외부 크레이트와 각각의 요구 버전을 카고에게 알려줍니다. 카고는 버전 명시의 표준인 [유의적 버전(Semantic Versioning)]()을 이해합니다. `0.8.5`는 실제로 `^0.8.5`의 축약형인데, 이는 최소 `0.8.5` 이상이지만 `0.9.0` 아래의 모든 버전을 의미합니다.

다시 프로젝트를 빌드합니다.
```shell
$ cargo build
    Updating crates.io index
  Downloaded libc v0.2.150
  Downloaded 1 crate (719.4 KB) in 1.42s
   Compiling libc v0.2.150
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.10
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/Users/xtring/Study/rust-study/rust-kr/02_guessing_game_tutorial/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 5.64s
```

Cargo는 `rand` 크레이트와 의존된 다른 크레이트들도 가져옵니다. 이것들을 함께 컴파일한 다음 사용 가능한 의존성과 함께 프로젝트를 컴파일합니다.

#### 크레이트를 새로운 버전으로 업데이트하기
```shell
$ cargo update
```
*Cargo.lock* 파일을 무시하고 Cargo.toml에 명시한 요구사항에 맞는 최신 버전을 확인합니다.
기본적으로 0.8.5 보다 크고 0.9.0 보다 작은 버전을 찾을 것입니다. 카고는 0.9.0 버전을 무시합니다. Cargo.lock 파일에서 변경이 일어난 것과 앞으로 사용될 rand 크레이트이 버전이 0.8.6임을 확인할 수 있습니다. 0.9.0 혹은 0.9.x 버전을 사용하고 싶다면 Cargo.toml을 수정해야합니다.  
그리고 다시 빌드하게되면 사용가능한 크레이트들의 레지스트리를 업데이트할 것이고 변경된 rand의 명시된 버전에 따라 다시 평가할 것입니다.

### 임의의 숫자 생성하기
*src/main.rs*를 아래와 같이 수정합니다.
```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```
- `use rand::Rng;` 라인을 추가
- `rand::thread_rng` 함수를 호출하여 OS가 시드(seed)를 정하고 현재 스레드에서만 사용되는 난수 생성기입니다.
  - 다음으로 `gen_range` 메서드를 호출합니다. 이 메서드는 `Rng` 트레이트에 정의되어있습니다.
    - `gen_range` 메서드는 범위 표현식을 인수로 받아서 해당 범위 내 임의의 숫자를 생성
      - `start..=end` 형태로 표현
  
자금은 게임을 시작하며 답을 출력해주고 있습니다. 그래도 한번 랜덤한 숫자를 출력하는지 확인해봅시다!
```shell
$ cargo run
Guess the number!
The secret number is: 24
Please input your guess.
6
You guessed: 6
```

## 비밀번호와 추릿값을 비교하기
랜덤으로 생성된 비밀번호와 사용자의 추릿값을 비교합니다.
*src/main.rs*를 수정합니다.
```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --생략--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```
먼저 use를 구문을 하나 더 사용하여 표준 라이브러리로부터 `std::cmp::Ordering`이라는 타입을 가져옵니다. `Ordering`은 열거형이고 `Less`, `Greater`, `Equal`이라는 배리언트들을 가지고 있습니다. 
`cmp` 메서드는 두 값을 비교하고 비교 가능한 모든 것들에 대해 호출할 수 있습니다. 이 메서드는 비교하고 싶은 값들의 참조자(`&`)를 받습니다.
  
여기서 `guess`와 `secret_number`를 비교하고 있습니다. `cmp`는 `Ordering` 열거형을 돌려줍니다. [match](https://rust-kr.github.io/doc.rust-kr.org/ch06-02-match.html) 표현식을 이용하여 `cmp`가 두 값을 비교한 결과인 Ordering의 값에 따라 무엇을 할 것인지 결정합니다.

해당 코드 추가 후 현재는 컴파일되지 않습니다.
```shell
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
   |                 |
   |                 arguments to this method are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
```
이 에러는 *일치하지 않는 타입(mismatched types)* 이 있음을 알려줍니다. 러스트는 강한 정적 타입 시스템을 가지고 있습니다. 타입 추론도 수행합니다.
위에서 `guess`는 `String::new()`를 통해 선언되었습니다. 따라서 `guess`는 `String` 타입입니다.

한편 `secret_number`는 정수형입니다.  
러스트의 정수형은 `i32`는 32비트 정수, `u32`는 32비트의 부호 없는 정수, `i64`는 64비트의 정수이며 그 외에도 비슷합니다. 다른 정수형임을 유추할 수 있는 타입 정보를 따로 추가하지 않는다면 러스트는 기본적으로 숫자들을 `i32`로 이해합니다.  
즉, 이 에러의 원인은 러스트가 문자열과 정수형을 비교할 수 없기 때문에 발생한 것입니다.  

`guess`를 정수형을 변환해보겠습니다.
- *src/main.rs*
```rust
// --생략--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```
`let guess: u32 = guess.trim().parse().expect("Please type a number!");`
여기에 `guess`라는 변수가 추가로 생성되었습니다. 러스트에서는 기존에 있던 변수를 새로운 값으로 가리는 (shadow) 것을 허용합니다. *섀도잉(shadowing)* 은 다른 변수명을 다시 선언하는 것이 아닌 기존의 변수명을 재사용할수 있도록 합니다.  
3장에서 더 자세하게 이야기 하겠지만, **어떤 한 타입의 값을 다른 타입으로 바꾸고 싶을 때 자주 사용되는 기능이라는 것을 알아두면 됩니다.**  
- `guess.trim().parse()`는 원래 있던 `guess`를 참조하고 String 인스턴스의 `trim` 메서드를 통해 처음과 끝부분의 공백문자를 제거한 뒤 `parse` 메서드를 통해 `let guess: u32`에 선언된 타입과 같이 변환합니다.
  - `trim` 메서드를 실행하는 이유는 사용자 값을 받고 난뒤 엔터에 따른 `\r\n`을 제거하기 위해 사용하는 것입니다. 
  
이를 통해 같은 타입(`u32`)을 가진 두 값이 비교됩니다.

> parse 메서드의 호출은 에러가 발생하기 쉽습니다. 예를 들어 A👍%과 같은 문자열이 포함되어 있다면 정수로 바꿀 방법이 없습니다. parse 메서드는 실패할 수도 있으므로, ‘Result 타입으로 잠재적 실패 다루기’에서 다루었던 read_line와 마찬가지로 Result 타입을 반환합니다. 이 Result는 expect 메서드를 사용하여 같은 방식으로 처리하겠습니다. 만약 parse 메서드가 문자열로부터 정수를 만들어 낼 수 없어 Err Result 배리언트를 반환한다면, expect 호출은 게임을 멈추고 제공한 메시지를 출력합니다. 만약 parse 메서드가 성공적으로 문자열을 정수로 바꾸었다면 Result의 Ok 배리언트를 돌려받으므로 expect에서 Ok에서 얻고 싶었던 값을 결과로 받게 됩니다.
  
이제 프로그램을 실행해봅시다.
```shell
$ cargo run
Guess the number!
The secret number is: 65
Please input your guess.
3
You guessed: 3
Too small!
```

현재 사용자는 추측값을 한번 추리 가능합니다. 이를 반복문을 통해 반복적인 추리가 가능할 수 있도록 해봅시다.

## 반복문을 이용하여 여러 번의 추리 허용하기
`loop` 키워드를 통해서 무한루프를 실행해볼 수 있습니다.
- *src/main.rs*
```rust
    // --생략--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --생략--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

그런데 사용자는 이 프로그램을 종료할 수 없을 것 같습니다.
사용자는 `ctrl + c` 단축키를 통해서 이 프로그램을 종료할 수 있습니다. 혹은 사용자 추리값으로 단순히 문자열을 입력하더라도 종료될 수 있습니다.

## 정답을 맞친 후 종료하기
사용자가 정답을 맞혔을 때 게임이 종료되도록 `break` 문을 추가합니다.
- *src/main.rs*
```rust
        // --생략--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```
You win! 뒤에 `break` 라인을 추가하면 사용자가 비밀번호를 맞혔을 때 프로그램이 루프를 종료하게 됩니다. 루프가 main의 마지막 부분이므로 루프의 종료는 프로그램의 종료를 의미합니다.

## 잘못된 입력값 처리하기
`guess`의 입력값으로 문자열이 들어오는 경우 이에 대한 에러 처리를 작성해봅시다.

```rust
        // --생략--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // --생략--
```
`expect` 메서드 호출을 `match` 표현식으로 바꾸어 에러 발생 시 즉시 종료가 아닌 에러 처리로 바꾸었습니다. `parse` 메서드가 `Result` 타입을 반환한다는 점, 그리고 `Result`는 `Ok`나 `Err` 배리언트를 가진 열거형임을 기억하세요. 여기서는 `cmp` 메서드의 `Ordering` 결과에서와 마찬가지로 `match` 표현식을 사용하고 있습니다.

이제 프로그램을 동작시켜봅시다!
```shell
$ cargo run
Guess the number!
The secret number is: 38
Please input your guess.
50
You guessed: 50
Too big!
Please input your guess.
13
You guessed: 13
Too small!
```

이제 마지막으로 비밀번호를 출력하는 `println!`를 삭제합니다.
완성!


