# 最終テーブル定義書

## 1. 共通ルール

- **ID**: `VARCHAR(21)` (NanoID: 英数字のみ、ハイフンなし)
- **監査カラム**: 全テーブルに `created_at`, `updated_at` を搭載。
- **数値型**: 金額は `DECIMAL(19, 4)`、比率は `DECIMAL(5, 2)` を使用。
- **命名規則**: PostgreSQL予約語を避け、`Account` をユーザー管理に使用。

---

## 2. テーブル一覧

### 2.1 `accounts` (ユーザー管理)

システムを利用する主体。PostgreSQL予約語回避のため `User` ではなく `Account`。

| カラム名 | 型 | 制約 | 説明 |
| :--- | :--- | :--- | :--- |
| `id` | VARCHAR(21) | PRIMARY KEY | NanoID |
| `name` | VARCHAR(100) | NOT NULL UNIQUE | ユーザー名 |
| `email` | VARCHAR(255) | NOT NULL UNIQUE | メールアドレス |
| `created_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |
| `updated_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |

### 2.2 `asset_categories` (ユーザー別資産クラス)

リバランスの計算単位。ユーザーごとに「アメリカの株式」等の枠と目標比率を設定。

| カラム名 | 型 | 制約 | 説明 |
| :--- | :--- | :--- | :--- |
| `id` | VARCHAR(21) | PRIMARY KEY | NanoID |
| `account_id` | VARCHAR(21) | NOT NULL REFERENCES accounts(id) | 所有ユーザー |
| `name` | VARCHAR(100) | NOT NULL | カテゴリ名 (例: アメリカの株式) |
| `target_ratio` | DECIMAL(5, 2) | NOT NULL | **目標構成比率 (%)** |
| `created_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |
| `updated_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |
| **UNIQUE** | (account_id, name) | | 同一ユーザー内での名称重複禁止 |

### 2.3 `asset_master` (共通銘柄マスタ)

世の中に存在する投資信託やETFの定義。これは全ユーザー共通のカタログ。

| カラム名 | 型 | 制約 | 説明 |
| :--- | :--- | :--- | :--- |
| `id` | VARCHAR(21) | PRIMARY KEY | NanoID |
| `name` | VARCHAR(255) | NOT NULL UNIQUE | 銘柄名 (例: eMAXIS Slim S&P500) |
| `ticker_symbol` | VARCHAR(20) | | |
| `created_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |
| `updated_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |

### 2.4 `user_asset_groupings` (ユーザー別グルーピング設定)

「どの銘柄を、どの資産クラスとして扱うか」をユーザーごとに定義する紐付け表。

| カラム名 | 型 | 制約 | 説明 |
| :--- | :--- | :--- | :--- |
| `id` | VARCHAR(21) | PRIMARY KEY | NanoID |
| `account_id` | VARCHAR(21) | NOT NULL REFERENCES accounts(id) | 所有ユーザー |
| `asset_master_id` | VARCHAR(21) | NOT NULL REFERENCES asset_master(id) | 銘柄参照 |
| `category_id` | VARCHAR(21) | NOT NULL REFERENCES asset_categories(id) | 所属カテゴリ |
| `created_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |
| `updated_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |
| **UNIQUE** | (account_id, asset_master_id) | | 1銘柄は1つのカテゴリのみに所属 |

### 2.5 `assets` (保有資産状況)

ユーザーが実際に「今、いくら持っているか」を記録。

| カラム名 | 型 | 制約 | 説明 |
| :--- | :--- | :--- | :--- |
| `id` | VARCHAR(21) | PRIMARY KEY | NanoID |
| `account_id` | VARCHAR(21) | NOT NULL REFERENCES accounts(id) | 所有ユーザー |
| `asset_master_id` | VARCHAR(21) | NOT NULL REFERENCES asset_master(id) | 銘柄参照 |
| `current_amount` | DECIMAL(19, 4) | NOT NULL | 現在の評価額 |
| `created_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |
| `updated_at` | TIMESTAMP | NOT NULL DEFAULT CURRENT_TIMESTAMP | |

---

## 3. リレーションシップ図（ER図）

---

## 4. 特徴

- **マルチユーザー対応**: `account_id` により、ユーザー間でのデータ混同を防ぎます。
- **自由なグルーピング**: `user_asset_groupings` を通じて、ユーザーAはVTIを「米国株」に、ユーザーBはVTIを「先進国株」に分類することが可能です。
- **計算の整合性**: `assets` に入っている金額を `user_asset_groupings` 経由で `asset_categories` ごとに集計することで、リバランス計算を行います。

---
