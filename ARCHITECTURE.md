# Architecture Documentation

## Project Overview

**Framework**: Actix  
**Language**: Rust  
**App Type**: saas  
**Quality Score**: 100.0/100

## Generated Components

Total Files: 28

### Generation Phases

- **core_implementation**: 2 files
- **data_layer**: 4 files
- **api_layer**: 4 files
- **auth_security**: 1 files
- **config_infrastructure**: 5 files
- **testing_quality**: 4 files
- **complete_validation**: 2 files
- **auto_fix**: 6 files

## Architecture Decisions

{
  "framework_knowledge": {
    "framework_name": "Actix",
    "language": "rust",
    "type": "api_framework",
    "constraints": [
      {
        "name": "memory_safety",
        "propagates_to": [
          "memory_management",
          "concurrency"
        ],
        "reason": "Rust guarantees memory safety at compile time",
        "MUST_FOLLOW": true
      },
      {
        "name": "async_runtime",
        "propagates_to": [
          "async_operations",
          "database_driver"
        ],
        "reason": "Actix uses Tokio async runtime",
        "MUST_FOLLOW": true
      }
    ],
    "strengths": [
      "Blazing fast (fastest web framework)",
      "Memory safe",
      "Zero-cost abstractions",
      "Fearless concurrency",
      "No garbage collector"
    ],
    "weaknesses": [
      "Steep learning curve",
      "Longer compile times",
      "Smaller ecosystem",
      "Rust's borrow checker",
      "Less rapid development"
    ],
    "best_for": [
      "Maximum performance requirements",
      "Safety-critical systems",
      "High-load APIs",
      "WebAssembly backends",
      "System-level services"
    ],
    "metadata": {
      "min_rust_version": "1.54",
      "performance_tier": "extreme",
      "learning_curve": "very_high"
    },
    "enforcement_rules": {
      "constraint_propagation": "All constraints must propagate to dependent components",
      "pattern_adherence": "Follow Actix idiomatic patterns strictly",
      "dependency_validation": "Verify all dependencies are compatible"
    }
  }
}

## Requirements Met

{
  "phase_0_complete": true,
  "intent_description": "Create an Actix Web backend for a cryptocurrency exchange with:\n- Real-time order book with WebSocket streaming\n- Order matching engine (price-time priority)\n- Wallet management with hot/cold storage separation\n- Trade history and portfolio tracking\n- KYC verification workflow\n- Two-factor authentication\n- PostgreSQL with SQLx\n- Redis for order book cache\n- Rate limiting per API tier\n- Prometheus metrics",
  "business_domain": "api_service",
  "framework": "Actix",
  "language": "Rust",
  "app_type": "api",
  "architecture_pattern": "Layered",
  "api_style": "REST",
  "authentication": "JWT",
  "authorization": "Simple",
  "entities": [
    {
      "name": "User",
      "description": "Application user/account for cryptocurrency exchange",
      "fields": [
        {
          "name": "email",
          "type": "string",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "username",
          "type": "string",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "password_hash",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "first_name",
          "type": "string",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "last_name",
          "type": "string",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "is_active",
          "type": "boolean",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "last_login",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "is_verified",
          "type": "boolean",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "two_factor_enabled",
          "type": "boolean",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "kyc_status",
          "type": "string",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "created_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "updated_at",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [
        {
          "from": "User",
          "to": "KYCVerification",
          "type": "one_to_one",
          "name": "kyc_verification"
        },
        {
          "from": "User",
          "to": "Wallet",
          "type": "one_to_many",
          "name": "wallets"
        },
        {
          "from": "User",
          "to": "Portfolio",
          "type": "one_to_one",
          "name": "portfolio"
        },
        {
          "from": "User",
          "to": "TwoFactorAuthentication",
          "type": "one_to_one",
          "name": "two_factor_auth"
        },
        {
          "from": "User",
          "to": "APIKey",
          "type": "one_to_many",
          "name": "api_keys"
        }
      ],
      "is_auth_model": true
    },
    {
      "name": "Order",
      "description": "Represents a trade order in the cryptocurrency exchange system.",
      "fields": [
        {
          "name": "order_id",
          "type": "string",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "user_id",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "currency_pair",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "order_type",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "price",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "quantity",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "status",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "created_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "updated_at",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "filled_quantity",
          "type": "decimal",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "side",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [
        {
          "from": "Order",
          "to": "OrderBook",
          "type": "one_to_many",
          "name": "orders"
        }
      ],
      "is_auth_model": false
    },
    {
      "name": "OrderBook",
      "description": "Real-time order book for cryptocurrency exchange, handling buy and sell orders.",
      "fields": [
        {
          "name": "id",
          "type": "integer",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "symbol",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "side",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "price",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "quantity",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "timestamp",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "user_id",
          "type": "integer",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "status",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [
        {
          "from": "OrderBook",
          "to": "Trade",
          "type": "one_to_many",
          "name": "trades"
        }
      ],
      "is_auth_model": false
    },
    {
      "name": "Trade",
      "description": "Represents a trade transaction on the cryptocurrency exchange.",
      "fields": [
        {
          "name": "trade_id",
          "type": "string",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "user_id",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "pair",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "price",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "amount",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "trade_type",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "timestamp",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "status",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "fee",
          "type": "decimal",
          "required": false,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [],
      "is_auth_model": false
    },
    {
      "name": "Wallet",
      "description": "Manages user cryptocurrency wallets, including hot and cold storage separation.",
      "fields": [
        {
          "name": "user_id",
          "type": "integer",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "balance",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "currency",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "storage_type",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "created_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "updated_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "is_active",
          "type": "boolean",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "last_accessed",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [
        {
          "from": "Wallet",
          "to": "Trade",
          "type": "one_to_many",
          "name": "trades"
        }
      ],
      "is_auth_model": false
    },
    {
      "name": "KYCVerification",
      "description": "KYC (Know Your Customer) verification details for users in the cryptocurrency exchange.",
      "fields": [
        {
          "name": "user_id",
          "type": "integer",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "verification_status",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "verification_date",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "document_type",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "document_number",
          "type": "string",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "document_image",
          "type": "text",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "submitted_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "reviewed_at",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "reviewed_by",
          "type": "string",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "comments",
          "type": "text",
          "required": false,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [],
      "is_auth_model": false
    },
    {
      "name": "TwoFactorAuthentication",
      "description": "Handles two-factor authentication settings and statuses for users in the cryptocurrency exchange.",
      "fields": [
        {
          "name": "user_id",
          "type": "integer",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "is_enabled",
          "type": "boolean",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "method",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "secret",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "backup_codes",
          "type": "json",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "last_verified_at",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "created_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "updated_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [],
      "is_auth_model": false
    },
    {
      "name": "Portfolio",
      "description": "User's cryptocurrency portfolio tracking information",
      "fields": [
        {
          "name": "user_id",
          "type": "integer",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "currency",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "amount",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "average_buy_price",
          "type": "decimal",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "current_value",
          "type": "decimal",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "created_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "updated_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [
        {
          "from": "Portfolio",
          "to": "Trade",
          "type": "one_to_many",
          "name": "trade_history"
        }
      ],
      "is_auth_model": false
    },
    {
      "name": "APIKey",
      "description": "API key for accessing the cryptocurrency exchange services",
      "fields": [
        {
          "name": "key",
          "type": "string",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "user_id",
          "type": "integer",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "created_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "expires_at",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "is_active",
          "type": "boolean",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "rate_limit",
          "type": "integer",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "permissions",
          "type": "json",
          "required": true,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [
        {
          "from": "APIKey",
          "to": "RateLimit",
          "type": "one_to_one",
          "name": "rate_limit"
        }
      ],
      "is_auth_model": false
    },
    {
      "name": "RateLimit",
      "description": "Defines the rate limiting rules for API access based on user tiers.",
      "fields": [
        {
          "name": "id",
          "type": "integer",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "api_tier",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "max_requests",
          "type": "integer",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "time_window",
          "type": "integer",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "created_at",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "updated_at",
          "type": "datetime",
          "required": false,
          "unique": false,
          "indexed": false
        },
        {
          "name": "is_active",
          "type": "boolean",
          "required": true,
          "unique": false,
          "indexed": true
        }
      ],
      "relationships": [],
      "is_auth_model": false
    },
    {
      "name": "Metric",
      "description": "Metrics for monitoring and analyzing cryptocurrency exchange performance",
      "fields": [
        {
          "name": "metric_name",
          "type": "string",
          "required": true,
          "unique": true,
          "indexed": true
        },
        {
          "name": "metric_value",
          "type": "decimal",
          "required": true,
          "unique": false,
          "indexed": false
        },
        {
          "name": "timestamp",
          "type": "datetime",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "exchange_id",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "metric_type",
          "type": "string",
          "required": true,
          "unique": false,
          "indexed": true
        },
        {
          "name": "additional_info",
          "type": "json",
          "required": false,
          "unique": false,
          "indexed": false
        }
      ],
      "relationships": [
        {
          "from": "Metric",
          "to": "User",
          "type": "one_to_many",
          "name": "metrics"
        }
      ],
      "is_auth_model": false
    }
  ],
  "core_features": [
    "```json",
    "[",
    "\"Real-time order book with WebSocket streaming\",",
    "\"Order matching engine (price-time priority)\",",
    "\"Wallet management with hot/cold storage separation\",",
    "\"Trade history and portfolio tracking\",",
    "\"KYC verification workflow\",",
    "\"Two-factor authentication\",",
    "\"PostgreSQL with SQLx\",",
    "\"Redis for order book cache\",",
    "\"Rate limiting per API tier\",",
    "\"Prometheus metrics\"",
    "]",
    "```"
  ],
  "database": "PostgreSQL",
  "cache": null,
  "queue": "Celery",
  "storage": null,
  "real_time": "WebSockets",
  "search": "Elasticsearch",
  "real_time_updates": true,
  "payment_integration": false,
  "file_uploads": false,
  "notifications": false,
  "third_party_integrations": [],
  "test_coverage_target": 70,
  "test_types": [
    "unit",
    "integration"
  ],
  "containerized": true,
  "orchestration": "Docker Compose",
  "ci_cd": "GitHub Actions",
  "compliance": [
    "GDPR"
  ],
  "scale_tier": "medium",
  "estimated_endpoints": 83,
  "estimated_files": 91,
  "estimated_cost": "$70/month",
  "estimated_timeline": "6 weeks"
}

## Framework-Specific Features

{
  "name": "Actix",
  "language": "Language.RUST",
  "framework_type": "FrameworkType.API_FRAMEWORK",
  "constraints": [
    "FrameworkConstraint(name='memory_safety', constraint_type='safety', required=True, value='compile_time_checks', propagates_to={'memory_management', 'concurrency'}, conflicts_with=set(), reason='Rust guarantees memory safety at compile time')",
    "FrameworkConstraint(name='async_runtime', constraint_type='concurrency_model', required=True, value='tokio', propagates_to={'async_operations', 'database_driver'}, conflicts_with=set(), reason='Actix uses Tokio async runtime')"
  ],
  "strengths": [
    "Blazing fast (fastest web framework)",
    "Memory safe",
    "Zero-cost abstractions",
    "Fearless concurrency",
    "No garbage collector"
  ],
  "weaknesses": [
    "Steep learning curve",
    "Longer compile times",
    "Smaller ecosystem",
    "Rust's borrow checker",
    "Less rapid development"
  ],
  "best_for": [
    "Maximum performance requirements",
    "Safety-critical systems",
    "High-load APIs",
    "WebAssembly backends",
    "System-level services"
  ],
  "metadata": {
    "min_rust_version": "1.54",
    "performance_tier": "extreme",
    "learning_curve": "very_high"
  }
}

## Validation Results

Errors Found: 0

No errors!

---

*Generated by OrchesityAI*  
*Universal Backend Generator*
