# 소유권 이해하기
> 소유권(ownership)은 러스트에서 가장 독특한 기능이며 언어 전반에 깊은 영향을 끼칩니다. 소유권은 러스트가 가비지 컬렉터 없이 메모리 안정성을 보장하도록 해주므로, 소유권이 어떻게 동작하는지를 이해하는 것은 중요합니다. 이 장에서는 소유권을 비롯해 소유권과 관련된 대여(borrowing), 슬라이스(slice) 기능, 그리고 러스트에서 데이터를 메모리에 저장하는 방법에 대해 알아보겠습니다.


## 소유권이 뭔가요?
소유권은 러스트 프로그램의 메모리 관리법을 지배하는 규칙 모음입니다.  
모든 프로그램은 작동하는 동안 컴퓨터의 메모리 사용 밥법을 관리해야 합니다.
- 몇몇 언어는 가비지 컬렉션으로 프로그램에서 더 이상 사용하지 않는 메모리를 정기적으로 찾고 정리하는 방식을 가짐
- 또 다른 언어는 프로그래머가 직접 명시적으로 메모리를 할당하고 해제하는 방식을 택함
- 러스트는 이 두 방식이 아닌 제 3의 방식을 택함
  - **`소유권(ownership)`이라는 시스템을 만들고, 컴파일러가 컴파일 중에 검사할 여러 규칙을 정해 메모리를 관리하는 방식**

### 스택 영역과 힙 영역
- 대부분의 프로그래밍 언어에서는 스택, 힙 영역을 주제로 고민할 필요가 많지 않음
  - 러스트와 같은 시스템 프로그래밍 언어에서는 값을 스택에 저장하느냐 힙에 저장하느냐의 차이가 프로그램의 동작 및 프로그램의 의사 결정에 큰 영향을 미침
- `스택`과 `힙` 둘 다 작성한 프로그램이 런타임에 이용하게 될 메모리 영역이라는 공통점을 가지지만, 구조는 각기 가름
  - `스택(stack)`: 값이 들어온 순서대로 저장, 역순으로 제거
    - 동작방식
      - 후입선출(LIFO; *last in, first out*)
      - 스택에 데이터를 추가하는 행위를 *푸시(push)* 라고 하며, 반대로 스택에서 데이터를 제거하는 행위는 *팝(pop)* 이라고 함
      - 스택에 저장되는 데이터는 모두 명확하고 크기가 정해져 있어야 함
    - **컴파일 타임에 크기를 알 수 없거나, 크기가 변경될 수 있는 데이터는 스택 대신 힙에 저장되어야함**
  - `힙(heap)`
    - 동작방식
      - 먼저 데이터를 힙에 넣을 때 먼저 저장할 공간이 있는지 운영체제에 물어봄
      - 메모리 할당자는 커다란 힙 영역 안에서 빈 지점을 찾고, 이 지점은 사용 중이라고 표시한 뒤 해당 지점을 가리키는 포인터(pointer)를 우리한테 반환
        - 이 과정을 *힙 공간 할당(allocating on the heap)*, 줄여서 *할당(allocation)* 이라고 함
          - 스택에 값을 푸시하는 것은 할당이라고 하지 않음
    - 포인터는 크기가 정해져 있어 스택에 저장할 수 있으나, 포인터가 가리키는 실제 데이터를 사용하고자 할 때는 포인터를 참조해 해당 포인터가 가리키는 위치로 이동하는 과정을 거침
- 스택 영역은 데이터에 접근하는 방식상 힙 영역보다 속도가 빠름
  - 메모리 할당자가 새로운 데이터를 저장할 공간을 찾을 필요 없이 항상 스택의 가장 위에 데이터를 저장하면 됨
  - 반면, 힙에 공간을 할당하는 직업은 좀 더 많은 작업을 요구함
    - 메모리 할당자가 데이터를 저장하기 충분한 공간을 먼저 찾고 다음 할당을 위한 준비를 위해 예약을 수행해야 하기 때문
- 힙 영역은 포인터가 가리키는 곳을 찾아가는 과정을 가지기 때문에 느림
  - 힙에 있는 데이터들은 서로 멀리 떨어져 있어 프로세서가 계속해서 돌아다녀야 하기 때문에 느림
  - 현대 프로세서는 메모리 내부를 이리저리 왔다 갔다 하는 작업이 적을 수록 빨라질수 있음
  - 힙 영역처럼 데이터가 서로 멀리 떨어져 있으며 작업이 느려지고, 반대로 스택 영역처럼 데이터가 서로 붙어 있으면 작업이 빨라짐
- 함수를 호출하면, 호출한 함수에 넘겨준 값(값 중엔 힙 영역의 데이터를 가리키는 포인터도 있을 수 있음)과 해당 함수의 지역 변수들이 스택에 푸시됨
  - 그리고 이 데이터들은 함수가 종료될 때 팝(제거)됩니다.
- 코드 어느 부분에서 힙의 어떤 데이터를 사용하는지 추적하고, 힙에서 중복되는 데이터를 최소화하고, 쓰지 않는 데이터를 힙에서 정리해 영역을 확보하는 작업은 모두 소유권과 관련있음
  - 즉, **소유권의 주요 목표가 힙 데이터의 관리라는 점을 알고 있으면 소유권의 동작 방식을 이해하는 데 도움**

### 소유권 규칙
다음 규칙을 명심합시다.
- 러스트에서, 각각의 값은 *소유자(owner)* 가 정해져 있음
- 한 값의 소유자는 동시에 여럿 존재할 수 없음
- 소유자가 스코프 밖으로 벗어날 때, 값은 버려짐(`dropped`)

### 변수의 스코프
- 소유권 예제 1) 변수의 스코프(scope)
  - 스코프란 프로그램 내에서 아이템이 유효한 범위를 말함
```rust
fn main() {
    {
        let s = "hello";    // 여기서 부터 s 변수가 유효한 지점
        // s로 어떤 작업을 함
    }                       // 이 스코프가 종료되었고, s가 더 이상 유효하지 않음
}
```
- 중요한 두 가지
  - `s`가 스코프 내에 나타나면 유효함
  - 유효기간은 스코프 밖으로 벗어나기 직전까지

### `String` 타입
여기서 `String`의 소유권과 관련된 부분에만 집중해보겠습니다. 이러한 관점은 표준 라이브러리가 제공하는 다른 타입들이나 만들어볼 복잡한 복잡한 데이터 타입에도 적용됩니다.
여태 보아온 문자열은 코드 내에 하드코딩하는 방식의 문자열 리터럴이었습니다. 문자열 리터럴은 쓰기 편리하지만, 만능은 아닙니다. 그이유는 문자열 리터럴이 불변성(immutable)을 지니기에 변경할 수 없다는 점과 프로그램에 필요한 모든 문자열을 우리가 프로그래밍하는 시점에 알 수는 없다는 점 때문입니다. 사용자한테 문자열을 입력받아 저장하는 기능 등을 만들어야 하는 상황에서는 문자열 리터럴을 사용할 수 없습니다.
따라서 러스트는 또 다른 문자열 타입인 `String`을 제공합니다. 이 타입은 힙에 할당된 데이터를 다루기 때문에, 컴파일 타임에 크기를 알 수 없는 텍스트도 저장할 수 있습니다.
`String` 타입은 다음과 같이 `from` 함수와 문자열 리터럴을 이용해 생성이 가능합니다.
```rust
let s = String::from("hello");
```
- 이중콜론`::`은 우리가 함수를 사용할 대 `string_from` 같은 함수명을 사용하지 않고 `String` 타입에 있는 특정된 `from` 함수라는 것을 지정할 수 있게 해주는 네임스페이스 연산자
  
이 `String` 문자열은 변경이 다음과 같이 변경 가능합니다.
```rust
let mut s = String::from("hello");
s.push_str(", world!");
println!("{}", s);  // hello, world!
```
이 처럼 문자열 리터럴과 `String`에 무슨 차이가 있어 어떤 것은 변경 가능하고 어떤 것은 변경할 수 없을까요? 차이점은 **각 타입의 메모리 사용 방식** 에 있습니다.


### 메모리와 할당
- 문자열 리터럴은 컴파일 타임에 내용을 알 수 있음
  - 텍스트가 최종 실행파일에 하드코딩됨
    - 이 방식은 빠르고 효율적이지만, 문자열이 변하지 않을 경우에만 사용가능
    - 컴파일 타임에 크기를 알 수 없고 실행 중 크기가 변할 수도 있는 텍스트는 바이너리 파일에 집어넣을 수 없음
- `String` 타입은 힙에 메모리를 할당하는 방식을 사용하기 때문에 텍스트 내용 및 크기를 변경할 수 있음
  - 실행 중 메모리 할당자로부터 메모리를 요청해야함
    - `String::from` 호출 시, 필요한 메모리를 요청하도록 구현되어 있음
      - 프로그래밍 언어 사이에서는 일반적으로 사용하는 방식
  - `String` 사용을 마쳤을 때 메모리를 해제할(즉, 할당자에게 메모리를 반납할) 방법이 필요
    - *가비지 컬렉터(garbage collector, GC)* 를 갖는 언어에서는 GC가 사용하지 않는 메모리를 찾아 없애주므로 프로그래머가 신경 쓸 필요가 없음
    - GC가 없는 대부분의 언어에서는 할당받는 메모리가 필요 없어지는 지점을 프로그래머가 직접 찾아 메모리 해제 코드를 작성해야 함
      - 이것은 많은 프로그래머가 실수하는 부분이며 많은 경우 버그를 일으킴
      - 따라서 `allocate`와 `free`가 하나씩 짝짓도록 만들어야함
    - 러스트에서는 이 문제를 **변수가 자신이 소속된 스코프를 벗어나는 순간 자동으로 메모리를 해제하는 방식으로 해결**
      - **러스트는 변수가 스코프 밖으로 벗어나면 `drop`이라는 특별한 함수를 호출함**
      - `drop` 함수는 닫힌 중괄호(`}`)가 나타나는 지점에서 자동으로 호출됨

### 변수와 데이터 간 상호작용 방식: 이동
```rust
let x = 5;
let y = x;
```
x와 y는 정수형 값으로 크기가 정해진 단순한 값이기 때문에 두 5값은 스택에 푸시됩니다.

```rust
let s1 = String::from("hello");
let s2 = s1;
```
힙 메모리에 `hello`를 저장하고 실제 s1은 메모리의 저장된 `hello` 문자열의 주소값을 저장합니다.
구체적으로는 포인터, 길이, 용량을 모두 가집니다. 이 데이터는 스택에 저장됩니다.
그리고 실제 `hello` 문자열은 힙 메모리에 저장됩니다.
  
- case1) s1을 s2에 대입하면 `String` 데이터가 복사됨
  - 이때 데이터는 스택에 있는 데이터, 즉 포인터, 길이, 용량 값을 말함
  - 포인터가 가리키는 힘 영역의 데이터가 복사되지 않음
  - 즉, s1, s2 `hello`가 위치한 메모리 위치를 가짐
- case2) 만약 s1, s2가 힙 메모리의 데이터까지 복사할 경우 러스트가 계속해서 이런 방식으로 동작한다면 힙 내 데이터가 커질 수록 `s2 = s1` 연산은 굉장히 느려질 것으로 예상됨

`case1` 처럼 두 포인터가 같은 곳을 가리킬 경우는 s2, s1이 스코프 밖으로 벗어날 때 각각 메모리를 해제하게 되면 *중복 해제(double free)* 에러가 발생할 것입니다.
이는 메모리 안정성 버그 중 하나이며 보안을 취약하게 만드는 메모리 손상의 원인입니다.
  

메모리 안정성을 보장하기 위해서, 러스트는 `let s2 = s1;` 라인 뒤로는 `s1`이 더 이상 유효하지 않다고 판단합니다.
이로써 러스트는 `s1`이 스코프를 벗어나더라도 아무것도 해제할 필요가 없습니다. 
`s2`가 만들어진 이후에 `s1`을 사용하는 경우 어떤 일이 생기는지 확인해보면, 작동하지 않음을 알 수 있습니다.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // error[E0382]: borrow of moved value: `s1`
    // 유효하지 않은 참조값의 사용을 감지했다는 에러 발생
}
```
러스트에서는 기존의 변수를 무효화하기 때문에 이를 복사가 아닌 *이동(move)* 라고 하며, 
앞선 코드는 *`s1`이 `s2`로 소유권이 이동되었다* 라고 표현합니다.
이로써 `s2`만이 유효하니, 스코프 밖으로 벗어났을 때 자신만 메모리를 해제할 것이고,
문제가 해결됩니다.  
실제로 러스트는 절대 자동으로 '깊은' 복사로 데이터를 복사하는 일은 없습니다.
따라서, 러스트가 자동으로 수행하는 모든 복사는 런타임 성능 측면에서 효율적이라 할 수 있습니다.

### 변수와 데이터 간 상호작용 방식: 클론
`String`의 힙 데이터까지 깊이 복사하고 싶을 땐 `clone`이라는 공용 메서드를 사용할 수 있습니다.
```rust
fn main() {
  let s1 = String::from("hello");
  let s2 = s1.clone();
  
  println!("s1 = {}, s2 = {}", s1, s2);
}
```

### 스택에만 저장되는 데이터: 복사
```rust
fn main() {
  let x = 5;
  let y = x;
  
  println!("x = {}, y ={}", x, y);
}
```
`clone`을 호출하지 않았는데 `x`는 계속해서 유효하며 `y`로 이동(move)되지도 않았습니다.  
이유는 정수형 등 컴파일 타임에 크기가 고정되는 타입은 모두 스택에 저장되기 때문입니다.
스택에 저장되니, 복사본을 빠르게 만들 수 있고, 따라서 굳이 `y`를 생성하고 나면 `x`를 무효화할 필요가 없습니다.
다시 말해 이런 경우엔 깊은 복사와 얕은 복사 간에 차이가 없습니다. 여기선 `clone`을 호출해도 얕은 복사와 차이가 없으니 생략해도 상관없습니다.
  
러스트에는 정수형처럼 스택에 저장되는 타입을 달아 놓을 수 있는 `Copy` 트레이트가 있습니다.
- 만약 어떤 타입에 이 `Copy` 트레이트가 구현되어 있다면, 이 타입의 변수는 사용되어도 이동(move)되지 않고 자명하게 복사되고, 대입연산 후에도 사용할 수 있음
- 구현하려는 타입이나, 구현하려는 타입 중 일부분에 `Drop` 트레이트가 구현된 경우엔 `Copy` 트레이트를 어노테이션 할 수 없음
- 스코프 밖으로 벗어났을 때 특정 동작이 요구되는 타입에 `Copy` 어노테이션을 추가하면 컴파일 에러가 발생

`Copy` 가능한 타입은?
> 일반적으로 단순한 스칼라 묶음은 `Copy` 가능하고, 할당이 필요하거나 리소스의 일종인 경우엔 불가능합니다.
- 모든 정수형 타입(예: `u32`)
- `true`, `false` 값을 갖는 논리 자료형 `bool`
- 모든 부동 소수점 타입(예: `f64`)
- 문자 타입 `char`
- `Copy` 가능한 타입만으로 구성된 튜플(예를 들어, `(i32, i32)`는 `Copy` 가능, `(i32, String)`은 불가능)

### 소유권과 함수
- 함수로 값을 전달하는 메커니즘은 변수에 값을 대입할 때와 유사함
  - 함수에 변수를 전달하면 대입 연산과 마찬가지로 이동이나 복사가 일어나기 때문
```rust
fn main() {
  let s = String::from("hello");
  
  takes_ownership(s); // s의 소유권은 takes_ownership으로 이동
  
  let x = 5;
  
  makes_copy(x);    // x는 함수로 이동(move)되지만 i32는 Copy이므로 앞으로 x를 계속해서 사용할 수 있음
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}   // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출됨
    // 메모리가 해제됨

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}   // 여기서 some_integer가 스코프 밖으로 벗어남, 별다른 일이 발생하지 않음
```
- `takes_ownership` 함수를 호출한 이후에 `s`를 사용하려할 경우, 컴파일 타임 에러를 발생시킴


### 반환 값과 스코프
소유권은 값을 반환하는 과정에서도 이동(move)합니다.
```rust
fn main() {
  let s1 = gives_ownership();   // gives_ownership이 자신의 반환값을 s1로 이동(move)
  
  let s2 = String::from("hello");   // s2가 스코프 안으로 들어옴
  
  let s3 = takes_and_gives_back(s2);    // s2는 takes_and_gives_back로 이동
                                        // 이 함수 또한 자신의 반환 값을 s3로 이동
}

fn gives_ownership() -> String {
  let some_string = String::from("yours");
  some_string   // some_string이 반환되고 호출자 함수 쪽으로 이동
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string  
}
```
- 상황은 다양할지라도, 변수의 소유권 규칙은 언제나 동일함
- 어떤 값을 다른 변수에 대입하면 값이 이동하고, 힙에 데이터를 갖는 변수가 스코프를 벗어나면, 사전에 해당 데이터가 이동하여 소유권이 다른 변수에 이동되지 않은 이상 `drop`에 의해 데이터가 제거됨

```rust
fn main() {
  let s1 = String::from("hello");
  
  let (s2, len) = calculate_length(s1);
  
  println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len()은 String의 길이를 반환

  (s, length)
}
```
- 하지만 이런 식은 일반적인 컨셉이 되기엔 너무 거추장스러움
- 러스트에는 소유권 이동 없이 값을 사용할 수 있는 *참조자(reference)* 라는 기능을 가지고 있음

sss