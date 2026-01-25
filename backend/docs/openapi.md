# OpenAPI 開発者ガイド

このプロジェクトでは，`utoipa` を使用して Rust Axum バックエンドから OpenAPI 仕様を自動生成しています．これにより，フロントエンドで型安全な API クライアントを簡単に生成できます．

## OpenAPI 仕様の確認方法

サーバーが起動している状態で，以下の URL にアクセスすることで仕様を確認できます．

- **Swagger UI**: [http://localhost:8000/swagger-ui](http://localhost:8000/swagger-ui)
  - ブラウザで API エンドポイントを試したり，リクエスト/レスポンスのスキーマを確認したりできます．
- **OpenAPI JSON**: [http://localhost:8000/api-docs/openapi.json](http://localhost:8000/api-docs/openapi.json)
  - プログラムから読み込むための生の JSON データです．

## フロントエンドでの利用方法

フロントエンド（TypeScript）では，`openapi-fetch` と `openapi-typescript` を組み合わせて使用することを推奨します．

### 1. 型定義の生成

`openapi-typescript` を使用して，バックエンドの仕様から TypeScript の型定義を生成します．

```bash
# プロジェクトのルート（またはフロントエンドディレクトリ）で実行
npx openapi-typescript http://localhost:8000/api-docs/openapi.json -o src/types/openapi.d.ts
```

### 2. API クライアントの作成

`openapi-fetch` を使用して，生成された型を反映した型安全なクライアントを作成します．

```typescript
import createClient from "openapi-fetch";
import type { paths } from "./types/openapi"; // 生成された型をインポート

const client = createClient<paths>({ baseUrl: "http://localhost:8000" });

// 使用例: 型安全な GET リクエスト
const { data, error } = await client.GET("/api/v1/catalog", {
  params: {
    // クエリパラメータなども型補完されます
  }
});

if (data) {
  console.log(data); // data は AssetMaster[] 型として推論されます
}
```

## バックエンドでの開発ルール

新しいエンドポイントを追加したり，モデルを変更したりした場合は，かならず以下の対応を行ってください．

1. **モデルの修正**: `#[derive(utoipa::ToSchema)]` を追加する．
2. **ハンドラの修正**: `#[utoipa::path(...)]` を追加し，リクエスト/レスポンスの型を記述する．
3. **ApiDoc への登録**: `src/main.rs` の `ApiDoc` 構造体の `paths` または `schemas` に追加する．

これにより，Swagger UI および JSON 仕様が自動的に更新されます．
