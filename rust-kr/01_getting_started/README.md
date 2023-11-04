`01`장에서 배우는 내용
- 각 운영체제(Linux, **macOS**, Windows) 별 러스트 설치법
- `Hello world!` 프로그램 작성하기
- 러스트 패키지 매니저 및 빌드 도구인 `cargo` 사용법

## 01-1. 러스트 설치
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