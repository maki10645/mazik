<script lang="ts">
  import FormWasm from "./FormWasm.svelte";
</script>

<div class="flex min-h-screen h-1 items-center justify-center">
  <div
    class="flex flex-col card h-[640px] preset-filled-surface-100-900 p-4 mx-8 border border-surface-500"
  >
    <h1 class="h3">MAZIK WebUIの使い方</h1>
    <p class="pt-8 font-motomachi">
      右にあるウィンドウのフォームを適切に入力し<br
      />Create4GoogleIMEかCreate4CorvusSKKかを選択していただくとダウンロードが開始します。<br
      />なお現在はGoogleIME用の物しか出力できないのでそんな選択肢はありません
    </p>
    <p class="pt-8 font-motomachi">以下はフォームの説明です</p>

    <hr class="hr border-surface-500" />

    <p>
      <br
      />・"シーケンス"とは割り当て先のボタンを押した時の動作を定義するものです。<br
      />・"割り当て先"は文字通りの意味です<br />シーケンス,
      割り当て先が複数個必要な場合はスペースで同数になるようにそれぞれ区切ってください。<br
      />・"ん"と"っ"は、それらを割り当てるキーを入力してください(そのキーを単打することで"ん"あるいは"っ"が出るようにします)<br
      />・通常のAZIKと同じく、"ん"に関しては"n"キーを２回押すことで出すようにすることを<br
      /> 選択可能にすることを思案していますが現状は出来ません。ご了承ください
    </p>
  </div>
  <span class="vr h-[620px] border-surface-500"></span>
  <div
    class="flex flex-col card w-[512px] h-[640px] preset-filled-surface-100-900 p-4 mx-8 border border-surface-500"
  >
    <FormWasm />
  </div>
</div>
