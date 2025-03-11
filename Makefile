# Set the default target of this Makefile
.PHONY: all
all:: ci

.PHONY: check-features
check-features:
	$(MAKE) -C crates/mso-sdk-types check-features
	$(MAKE) -C crates/mso-crypto check-features

.PHONY: check-fmt
check-fmt:
	cargo fmt -- --config imports_granularity=Item --check

.PHONY: fmt
fmt:
	cargo fmt -- --config imports_granularity=Item

.PHONY: clippy
clippy:
	cargo clippy --all-features --all-targets

.PHONY: test
test:
	cargo nextest run --all-features -p mso-sdk-types -p mso-crypto
	cargo test --doc

package_%.json: crates/mso-transaction-builder/tests/%/Move.toml crates/mso-transaction-builder/tests/%/sources/*.move
	cd crates/mso-transaction-builder/tests/$(*F) && mso move build --ignore-chain --dump-bytecode-as-base64 > ../../$@

.PHONY: test-with-localnet
test-with-localnet: package_test_example_v1.json package_test_example_v2.json
	cargo nextest run -p mso-graphql-client -p mso-transaction-builder

.PHONY: wasm
wasm:
	$(MAKE) -C crates/mso-sdk-types wasm
	$(MAKE) -C crates/mso-crypto wasm

.PHONY: doc
doc:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps

.PHONY: doc-open
doc-open:
	RUSTDOCFLAGS="--cfg=doc_cfg -Zunstable-options --generate-link-to-definition" RUSTC_BOOTSTRAP=1 cargo doc --all-features --no-deps --open

.PHONY: ci
ci: check-features check-fmt test wasm

.PHONY: ci-full
ci-full: ci doc

.PHONY: clean
clean:
	cargo clean

.PHONY: clean-all
clean-all: clean
	git clean -dX
