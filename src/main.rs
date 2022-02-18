fn main() {
    // 1.2 Formatted print
    // 보통, {} 형에 상관없이 자동 변환된다.
    println!("{} days", 31);

    // 퀄리브라켓을 쓸때 여러 옵션들이 있다.
    // args 숫자 사용
    println!("{0}, this is {1}. {1}, this is {0}", "Nick", "Jessy");

    // args 이름으로도 사용가능 하다.
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // :이후에 특별한 포메팅 사용가능
    println!("{} of {:b} people know binary, the other half doesn't", 1, 5);

    // 들여쓰기
    // 6칸부터 글씨가 나온다.
    println!("{number:>width$}", number=1, width=6);

    // pad numbers
    println!("{number:0>width$}", number=1, width=6);

    //



    // 1.1 Comments
    /*let x = 5 +/* 90 +*/ 5;
    println!("Is `x` 10 or 100? x = {}", x);*/



    // 1. Hello World
    /*println!("Hello, world!");
    println!("I'm Rustacean!");*/
}
