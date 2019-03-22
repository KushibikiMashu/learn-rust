// 単体テストは、テスト対象となるコードと共に、srcディレクトリの各ファイルに置きます。
// 慣習は、各ファイルにtestsという名前のモジュールを作り、テスト関数を含ませ、
// そのモジュールをcfg(test)で注釈することです。

// 非公開関数はテスト可能
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

// internal_adder関数はpubとマークされていないものの、テストも単なるRustのコードであり、
// testsモジュールもただのモジュールでしかないので、
// テスト内でinternal_adderを普通にインポートし呼び出すことができます。
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

//結合テストを作成するには、 まずtestsディレクトリが必要になります。