# Rust 프로젝트 생성하기

```shell
$ cargo new hello
    Create binary (application) `hello` package
```

`cargo new <package-name>` 명령어를 입력하면 새로운 러스트 패키지를 생성할 수 있다. `Cargo`가 만들어주는 패키지에는 적절한 값으로 미리 설정해 둔 표준 메타데이터 몇 가지가 함께 제공되어 매우 편리하다.

생성된 프로젝트의 최상위 구성은 아래와 같다.
```shell
# ./hello
$ ls -al
total 16
drwxr-xr-x  5 xtring  staff  160 Oct 29 23:47 .
drwxr-xr-x  6 xtring  staff  192 Oct 29 23:43 ..
-rw-r--r--  1 xtring  staff  174 Oct 29 23:43 Cargo.toml
-rw-r--r--  1 xtring  staff  384 Oct 29 23:47 README.md
drwxr-xr-x  3 xtring  staff   96 Oct 29 23:43 src
```

`Cargo`가 만든 파일 중에서 Cargo.toml은 패키지의 메타데이터를 담아 두기 위한 용도로 쓰인다.
아직은 특별히 추가된 페키지 없어 별 내용이 없다.

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2021"

[dependencies]
```
hello 프로그램이 언젠가 다른 라이브러리에 대한 의존성이 생길 때 관련 내용이 이곳에 기록된다.
그럼, `Cargo`가 대신 해당 라이브러리를 내려받고, 빌드하고, 업데이트해 줄 것이다.(Node.js에서 NPM과 같은 역할)

또, `Cargo`는 패키지를 만들 때 git 버전 관리 시스템을 쓸 수 있도록 .git 메타데이터 디렉터리와 .gitignore 파일을 같이 만들어 준다. 
만약 이 단계를 뛰어 넘고 싶은 경우 프로젝트 생성 시`$ cargo new <package-name> --vcs none`과 같이 입력하여 사용하면 된다.

프로그램을 빌드하기 위해서는 아래 명령어를 사용한다.
```shell
$ cargo run
Compiling hello v0.1.0 (/Users/xtring/Study/rust-study/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 4.99s
     Running `target/debug/hello`
Hello, world!
```

그러면 `Cargo`가 러스트 컴파일러인 rustc로 패키지를 컴파일한 뒤, 만들어진 실행 파일을 실행시켜준다.  
실행 파일은 패키지의 최상위에 있는 `target` 디렉터리에 만들어진다.
```shell
$ ls -l ../target/debug
```

작업을 마친 뒤에 생성된 파일을 정리하는 명령도 `Cargo`를 통해서 할수 있다.
```shell
$ cargo clean
```
컴파일된 파일이 제거된다.
