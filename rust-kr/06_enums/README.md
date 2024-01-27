# 열거형과 패턴 매칭  

> 이 장에서는 rust의 열거형이 데이터와 함께 의미를 담을 수 있는지 알아봅니다.

- 열거형(enumerations; enums)
  - 하나의 타입이 가질 수 있는 배리언트(variant) 들을 열거함으로써 타입을 정의할 수 있도록 함
- `Option`: 값이 어떤 것일 수도 있고 아무것도 아닐수도 있음을 표현하는 열거형
- `match`: 패턴 매칭을 통해 열거형의 값에 따라 다른 코드를 쉽게 실행할 수 있음
- `if let`: 열거형을 편하고 간결하게 다루기 위한 관용 표현 구문