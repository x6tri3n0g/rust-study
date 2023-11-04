`01`장에서 배우는 내용
- 각 운영체제(Linux, **macOS**, Windows) 별 러스트 설치법
- `Hello world!` 프로그램 작성하기
- 러스트 패키지 매니저 및 빌드 도구인 `cargo` 사용법

## 러스트 설치
이 문서에서는 러스트의 버전 및 러스트 관련 도구를 관리하는 커멘드 라인 도구인 `rustup`을 이용합니다.

[Rust 공식 홈페이지](https://www.rust-lang.org/)의 [Install Rust](https://www.rust-lang.org/tools/install) 혹은 [rustup.rs](https://rustup.rs/)를 살펴봅시다.

stable 버전의 러스트를 설치하는데 사용할 `rustup` 도구를 설치하는 명령어 입니다.
```shell
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
Rust is installed now. Great!
```

링커 이슈가 발생하는 경우 mocOS에서는 아래 명령어를 통해서 C 컴파일러를 설치할 수 있습니다. (혹은 xcode 최신버전으로 업데이트!)
```shell
$ xcode-select --install
```

- **설치 확인**
```shell
$ rustc --version
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

- 업데이트
```shell
$ rustup update
```

- 삭제
```shell
$ rustup self uninstall
```

- 로컬 문서 확인하기
```shell
$ rustup doc
```
러스트 설치시 로컬 문서도 함께 설치됩니다. 오프라인 상태로도 이용이 가능하다. 표준 라이브러리에서 제공하는 타입이나 함수 중 확인이 필요한 경우 API 문서를 확인해봅시다.


## Hello, World!
*초반 실습 부분 건너뜀*

### 컴파일과 실행은 별개의 과정입니다.
main.rs를 컴파일하는 과정이 있다. `rustc`를 사용
```shell
$ rustc main.rs
```

러스트는 소스 파일 컴파일에 성공하면 실행 가능한 바이너리를 만들어 냅니다.

간단한 프로그램에는 `rustrc`를 사용합니다. 다만 프로젝트가 커질수록 관리할 옵션이 많아지고, 코드 배포도 점점 번거러워집니다. 다음 내용에서 사용할 `카고(Cargo)`가 이러한 문제를 해결할 수 있습니다.


## 카고를 사용해봅시다
`카고(Cargo)`는 러스트 빌드 시스템 및 패키지 매니저입니다. 이 도구는 코드 빌드나, 코드 작성에 필요한 외부 라이브러리를 다운로드할 때나, 라이브러리를 제작할 때 겪는 귀찮은 일들을 상당수 줄여주는 편리한 도구입니다.

앞선 간단한 프로그램에서는 의존성을 추가하지 않았습니다. 훗날 복잡한 프로그램을 작성하게 되면 의존성 추가하게 됩니다. 의존성이 추가될 수록 카고가 이것을 도와줍니다.

앞서 [설치 절](## 러스트 설치)에서 카고는 설치되었습니다.
```shell
$ cargo --version
cargo 1.73.0 (9c4383fb5 2023-08-26)
```

### 카고로 프로젝트 생성하기
```shell
# cargo new <project-name>
$ cargo new hello_cargo
```
`cargo new <project-name>`를 통해 프로젝트를 생성했다면 `Cargo.toml` 파일과 `src` 디렉터리를 확인할 수 있습니다. 그 외에도 .gitignore 파일과 함께 새 Git 저장소가 초기화됩니다. 이 동작은 `cargo new --vcs=git` 명령을 통해 덮어쓸 수 있습니다.

`Cargo.toml`를 살펴봅시다.
- `[package]`는 섹션 헤더로 뒤에 패키지 설정 구문들이 따라옵니다. 나중에 우리가 이 파일에 내용을 추가하며 새로운 색션을 만들어 갑니다.
    - `[package]` 다음 세 라인은 카고가 코드를 컴파일하는데 필요한 설정 정보로, 각각 패키지명, 버전, 작성자, 사용하는 러스트 에디션을 뜻합니다.
- `[dependencies]`는 프로젝트에서 사용하는 의존성 목록입니다. 러스트에서는 코드 패키지를 크레이트(*crate*)라고 합니다.

### 카고로 프로젝트를 빌드하고 실행하기
#### 프로젝트 빌드
```shell
$ cargo build
```
프로젝트의 *target/debug/hello_cargo*에 실행 파일을 생성합니다. 기본 빌드가 디버그 빌드기 때문에, 카고는 *debug*라는 디렉터리 바이너리를 생성합니다.

#### 빌드된 파일 실행하기
```shell
$ ./target/debug/hello_cargo
Hello, world!
```
처음 `cargo build`를 실행하게 되면 최상위 디렉터리에 `Cargo.lock` 파일이 생성됩니다. 이는 프로젝트 의존성을 정확한 버전에서 실행시키기 위해서 자동으로 기록해 두는 파일입니다.

#### 빌드(컴파일) + 실행
이렇게 **빌드(컴파일) 후 실행**을 할수도 있지만 한번에 실행까지 수행하는 방법도 있습니다.
```shell
$ cargo run
Hello, world!
```

#### 실행파일 생성하지 않고 컴파일만하기
```shell
$ cargo check
```
이 명령어는 `cargo build` 보다 빠르고 실행 파일을 만들지 않아 코드에서 컴파일 문제가 발생하지 않는지 확인합니다. 실행 파일이 필요한 경우에만 `cargo build`를 사용 합니다.

#### 릴리즈 빌드 생성하기
프로젝트를 완성해서 배포(릴리즈)할 줌비가 끝났다면, `cargo build --release`를 사용해 릴리즈 빌드를 생성할 수 있습니다. 일반 빌드와 차이점으로는 *target/debug*가 아닌 *target/release*에 실행 파일이 생성됩니다. 추가로 최적화를 진행하여 컴파일이 오래 걸리는 대신 러스트 코드가 더 빠르게 작동합니다.
따라서, **작성한 코드 작동 속도를 벤치마킹할 시에는 릴리즈 빌드를 기준으로 해야 합니다.**

