---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "MAZIK"
  text: "日本語入力を改善しようMAZIKで"
  tagline: 面倒なAZIKの設計, 拡張, 設定をMAZIKで。
  actions:
    - theme: brand
      text: WebUI
      link: "https://google.com"

features:
  - title: WEBから
    details: WEBUIからフォームを入力するだけでGoogleIME, CorvusSKK用のローマ字テーブルを生成, ダウンロード
  - title: CUIから
    details: 元よりCLIAppとして開発していたので、CLIでも簡単にGoogleIME, CorvusSKK用のローマ字テーブルを生成
  - title: APIでも
    details: JsonをPOSTするだけでGoogleIME, CorvusSKK用のローマ字テーブルを生成
---

---

[[toc]]


MAZIK - Make AZIK はAZIKの拡張, 設計, 作成を支援するためのアプリケーションです。
Web, CLIから使用可能です

## 前提知識

### AZIKとは

ローマ字テーブルを改変することで、日本語をローマ字入力する上での打鍵数を凡そ30%削減できます。

### 新配列

Qwerty, JISかな配列などの非効率なキーボード配列以外のキーボード配列
Dvorak, 大西配列, 薙刀配列などがある

ローマ字配列と日本語配列の二種類があり当アプリケーションはローマ字配列を前提としています

## 使用方法


