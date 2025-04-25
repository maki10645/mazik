# azik_generator改めMAZIK

## このリポジトリについて

これらは旧azik_geretorをRust, wasm, SvelteKitで書き直してモノレポ擬きにしたリポジトリです
正直モノレポがなんたるかをなにひとつ理解してませんが御愛嬌

## 各ディレクトリの説明

- Core: Rustで書かれたwasm化されるコード群です。
- Front: SvelteKitで書かれたWebUIです
- Documents: VitePressで書かれる予定のドキュメントですが、簡単な設定はFrontの左側に表示させるので普通の所謂AZIKと私が設計したMakiyukiAZIK4Ohnishiの設定例を乗せる程度になると思います。

## 課題

- [ ] CorvusSKK対応
- [ ] 通常のAZIKと同じ様に"ん"を"nn"で打てるようにするオプションの追加
