#  (The MIT License)
#
#  Copyright (c) 2024 Mamadou Babaei
#
#  Permission is hereby granted, free of charge, to any person obtaining a copy
#  of this software and associated documentation files (the "Software"), to deal
#  in the Software without restriction, including without limitation the rights
#  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
#  copies of the Software, and to permit persons to whom the Software is
#  furnished to do so, subject to the following conditions:
#
#  The above copyright notice and this permission notice shall be included in
#  all copies or substantial portions of the Software.
#
#  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
#  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
#  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
#  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
#  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
#  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
#  THE SOFTWARE.


################################################################################
# Generic Settings
################################################################################

WEBRTC_LEAK_DEMO_PROJECT_NAME	:=	"Web RTC Leak Demo"

WEBRTC_LEAK_DEMO_ROOT_DIR	:=	"$(CURDIR)"

################################################################################
# Platform Detection
################################################################################

WEBRTC_LEAK_DEMO_BUILD_ARCH_AMD64		:=	amd64
WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD	:=	freebsd
WEBRTC_LEAK_DEMO_BUILD_PLATFORM_LINUX	:=	linux
WEBRTC_LEAK_DEMO_BUILD_PLATFORM_WINDOWS	:=	windows

ifeq ($(OS),Windows_NT)
WEBRTC_LEAK_DEMO_BUILD_PLATFORM	:=	$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_WINDOWS)
# Running a 32-bit (x86) Command Prompt sets PROCESSOR_ARCHITECTURE to x86,
# and instead defines PROCESSOR_ARCHITEW6432 for 64-bit Windows detection.
ifeq ($(PROCESSOR_ARCHITEW6432),AMD64)
WEBRTC_LEAK_DEMO_BUILD_ARCH	:=	$(WEBRTC_LEAK_DEMO_BUILD_ARCH_AMD64)
else # ($(PROCESSOR_ARCHITEW6432),AMD64)
ifeq ($(PROCESSOR_ARCHITECTURE),AMD64)
WEBRTC_LEAK_DEMO_BUILD_ARCH	:=	$(WEBRTC_LEAK_DEMO_BUILD_ARCH_AMD64)
endif # ($(PROCESSOR_ARCHITECTURE),AMD64)
endif # ($(PROCESSOR_ARCHITEW6432),AMD64)
else # ($(OS),Windows_NT)
WEBRTC_LEAK_DEMO_PLATFORM_UNAME_ARCH	:=	$(shell uname -m)
ifeq ($(WEBRTC_LEAK_DEMO_PLATFORM_UNAME_ARCH),amd64)
WEBRTC_LEAK_DEMO_BUILD_ARCH	:=	$(WEBRTC_LEAK_DEMO_BUILD_ARCH_AMD64)
else ifeq ($(WEBRTC_LEAK_DEMO_PLATFORM_UNAME_ARCH),x86_64)
WEBRTC_LEAK_DEMO_BUILD_ARCH	:=	$(WEBRTC_LEAK_DEMO_BUILD_ARCH_AMD64)
endif # ($(WEBRTC_LEAK_DEMO_PLATFORM_UNAME_ARCH),amd64)
WEBRTC_LEAK_DEMO_PLATFORM_UNAME	:=	$(shell uname -s)
ifeq ($(WEBRTC_LEAK_DEMO_PLATFORM_UNAME),FreeBSD)
WEBRTC_LEAK_DEMO_BUILD_PLATFORM	:=	$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD)
else ifeq ($(WEBRTC_LEAK_DEMO_PLATFORM_UNAME),Linux)
WEBRTC_LEAK_DEMO_BUILD_PLATFORM	:=	$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_LINUX)
endif # ($(WEBRTC_LEAK_DEMO_PLATFORM_UNAME),FreeBSD)
endif # ($(OS),Windows_NT)

################################################################################
# Target Settings
################################################################################

WEBRTC_LEAK_DEMO_TARGET_NAME			:=	webrtc-leak-demo
WEBRTC_LEAK_DEMO_TARGET_NAME_FREEBSD	:=	$(WEBRTC_LEAK_DEMO_TARGET_NAME)
WEBRTC_LEAK_DEMO_TARGET_NAME_LINUX		:=	$(WEBRTC_LEAK_DEMO_TARGET_NAME)
WEBRTC_LEAK_DEMO_TARGET_NAME_WINDOWS		:=	$(WEBRTC_LEAK_DEMO_TARGET_NAME).exe

WEBRTC_LEAK_DEMO_SOURCE_DIR		:=	"$(WEBRTC_LEAK_DEMO_ROOT_DIR)"

WEBRTC_LEAK_DEMO_MANIFEST_FILE	:=	"$(WEBRTC_LEAK_DEMO_SOURCE_DIR)"/Cargo.toml

################################################################################
# Build Settings
################################################################################

WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG	:=	debug
WEBRTC_LEAK_DEMO_BUILD_TYPE_RELEASE	:=	release

WEBRTC_LEAK_DEMO_BUILD_TYPE	?=	$(WEBRTC_LEAK_DEMO_BUILD_TYPE_RELEASE)

WEBRTC_LEAK_DEMO_BUILD_FLAGS	:=

WEBRTC_LEAK_DEMO_TARGET_ROOT_DIR	:=  "$(WEBRTC_LEAK_DEMO_ROOT_DIR)"/target

ifeq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_RELEASE))
WEBRTC_LEAK_DEMO_BUILD_FLAGS	:=	$(WEBRTC_LEAK_DEMO_BUILD_FLAGS) --release
endif # ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_RELLEASE))

WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_FREEBSD	:=	x86_64-unknown-freebsd
WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_LINUX		:=	x86_64-unknown-linux-gnu
WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_WINDOWS	:=	x86_64-pc-windows-msvc

ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))
WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE	:=	$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_FREEBSD)
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_LINUX))
WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE	:=	$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_LINUX)
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_WINDOWS))
WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE	:=	$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_WINDOWS)
endif # ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))

ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))
WEBRTC_LEAK_DEMO_TARGET_DIR_DEBUG	:=	"$(WEBRTC_LEAK_DEMO_TARGET_ROOT_DIR)"/"$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_FREEBSD)"/debug
WEBRTC_LEAK_DEMO_TARGET_DIR_RELEASE	:=	"$(WEBRTC_LEAK_DEMO_TARGET_ROOT_DIR)"/"$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_FREEBSD)"/release
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_LINUX))
WEBRTC_LEAK_DEMO_TARGET_DIR_DEBUG	:=	"$(WEBRTC_LEAK_DEMO_TARGET_ROOT_DIR)"/"$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_LINUX)"/debug
WEBRTC_LEAK_DEMO_TARGET_DIR_RELEASE	:=	"$(WEBRTC_LEAK_DEMO_TARGET_ROOT_DIR)"/"$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_LINUX)"/release
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_WINDOWS))
WEBRTC_LEAK_DEMO_TARGET_DIR_DEBUG	:=	"$(WEBRTC_LEAK_DEMO_TARGET_ROOT_DIR)"/"$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_WINDOWS)"/debug
WEBRTC_LEAK_DEMO_TARGET_DIR_RELEASE	:=	"$(WEBRTC_LEAK_DEMO_TARGET_ROOT_DIR)"/"$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE_WINDOWS)"/release
endif # ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))

ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))
WEBRTC_LEAK_DEMO_TARGET_BIN_DEBUG	:=	"$(WEBRTC_LEAK_DEMO_TARGET_DIR_DEBUG)"/"$(WEBRTC_LEAK_DEMO_TARGET_NAME_FREEBSD)"
WEBRTC_LEAK_DEMO_TARGET_BIN_RELEASE	:=	"$(WEBRTC_LEAK_DEMO_TARGET_DIR_RELEASE)"/"$(WEBRTC_LEAK_DEMO_TARGET_NAME_FREEBSD)"
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_LINUX))
WEBRTC_LEAK_DEMO_TARGET_BIN_DEBUG	:=	"$(WEBRTC_LEAK_DEMO_TARGET_DIR_DEBUG)"/"$(WEBRTC_LEAK_DEMO_TARGET_NAME_LINUX)"
WEBRTC_LEAK_DEMO_TARGET_BIN_RELEASE	:=	"$(WEBRTC_LEAK_DEMO_TARGET_DIR_RELEASE)"/"$(WEBRTC_LEAK_DEMO_TARGET_NAME_LINUX)"
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_WINDOWS))
WEBRTC_LEAK_DEMO_TARGET_BIN_DEBUG	:=	"$(WEBRTC_LEAK_DEMO_TARGET_DIR_DEBUG)"/"$(WEBRTC_LEAK_DEMO_TARGET_NAME_WINDOWS)"
WEBRTC_LEAK_DEMO_TARGET_BIN_RELEASE	:=	"$(WEBRTC_LEAK_DEMO_TARGET_DIR_RELEASE)"/"$(WEBRTC_LEAK_DEMO_TARGET_NAME_WINDOWS)"
endif # ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))

################################################################################
# Stage Settings
################################################################################

WEBRTC_LEAK_DEMO_STAGE_DIR	:=	"$(WEBRTC_LEAK_DEMO_ROOT_DIR)"/stage

################################################################################
# Version and Build Information
################################################################################

WEBRTC_LEAK_DEMO_GIT_TREE_STATE_DIRTY	:=	dirty

WEBRTC_LEAK_DEMO_GIT_BRANCH			:=	$(shell git rev-parse --abbrev-ref HEAD)
WEBRTC_LEAK_DEMO_GIT_TAG			:=	$(shell git describe --tags --abbrev=0 2>/dev/null)
WEBRTC_LEAK_DEMO_GIT_COMMIT			:=	$(shell git rev-parse --short HEAD)
WEBRTC_LEAK_DEMO_GIT_COMMIT_NUMS_SINCE_TAG	:=	$(shell git rev-list --count HEAD ^`git describe --tags --abbrev=0`)
WEBRTC_LEAK_DEMO_GIT_TREE_STATE			:=	$(shell test -n "`git status --porcelain`" && echo "dirty" || echo "clean")

ifeq ($(strip $(WEBRTC_LEAK_DEMO_GIT_TAG)),)
ifeq ($(strip $(WEBRTC_LEAK_DEMO_GIT_COMMIT)),)
WEBRTC_LEAK_DEMO_VERSION	:=	{VERSION_UNKNOWN}
else
WEBRTC_LEAK_DEMO_VERSION	:=	$(WEBRTC_LEAK_DEMO_GIT_COMMIT)
endif # ($(strip $(WEBRTC_LEAK_DEMO_GIT_TAG)),)
else # ($(strip $(WEBRTC_LEAK_DEMO_GIT_TAG)),)
WEBRTC_LEAK_DEMO_VERSION	:=	$(WEBRTC_LEAK_DEMO_GIT_TAG)
ifeq ($(filter $(WEBRTC_LEAK_DEMO_GIT_COMMIT_NUMS_SINCE_TAG), $(shell seq 0 9)),)
ifneq ($(strip $(WEBRTC_LEAK_DEMO_GIT_COMMIT_NUMS_SINCE_TAG)),)
ifneq ($(WEBRTC_LEAK_DEMO_GIT_COMMIT_NUMS_SINCE_TAG),0)
WEBRTC_LEAK_DEMO_VERSION	:=	$(WEBRTC_LEAK_DEMO_VERSION)-$(WEBRTC_LEAK_DEMO_GIT_COMMIT_NUMS_SINCE_TAG)
endif # ($(WEBRTC_LEAK_DEMO_GIT_COMMIT_NUMS_SINCE_TAG),0)
endif # ($(strip $(WEBRTC_LEAK_DEMO_GIT_COMMIT_NUMS_SINCE_TAG)),)
endif # ($(filter $(WEBRTC_LEAK_DEMO_GIT_COMMIT_NUMS_SINCE_TAG), $(shell seq 0 9)),)
WEBRTC_LEAK_DEMO_VERSION	:=	$(WEBRTC_LEAK_DEMO_VERSION)-$(WEBRTC_LEAK_DEMO_GIT_COMMIT)
endif # ($(strip $(WEBRTC_LEAK_DEMO_GIT_TAG)),)

ifeq ($(WEBRTC_LEAK_DEMO_GIT_TREE_STATE),$(WEBRTC_LEAK_DEMO_GIT_TREE_STATE_DIRTY))
WEBRTC_LEAK_DEMO_VERSION	:=	$(WEBRTC_LEAK_DEMO_VERSION)-modified
endif # ($(WEBRTC_LEAK_DEMO_GIT_TREE_STATE),$(WEBRTC_LEAK_DEMO_GIT_TREE_STATE_DIRTY))

WEBRTC_LEAK_DEMO_VERSION_MAJOR = $(shell echo $(WEBRTC_LEAK_DEMO_GIT_TAG) | cut -d '.' -f 1 | tr -d -c 0-9)
ifeq ($(WEBRTC_LEAK_DEMO_VERSION_MAJOR),)
WEBRTC_LEAK_DEMO_VERSION_MAJOR	:= 0
endif

WEBRTC_LEAK_DEMO_VERSION_MINOR = $(shell echo $(WEBRTC_LEAK_DEMO_GIT_TAG) | cut -d '.' -f 2 | tr -d -c 0-9)
ifeq ($(WEBRTC_LEAK_DEMO_VERSION_MINOR),)
WEBRTC_LEAK_DEMO_VERSION_MINOR	:= 0
endif

WEBRTC_LEAK_DEMO_VERSION_PATCH = $(shell echo $(WEBRTC_LEAK_DEMO_GIT_TAG) | cut -d '.' -f 3 | tr -d -c 0-9)
ifeq ($(WEBRTC_LEAK_DEMO_VERSION_PATCH),)
WEBRTC_LEAK_DEMO_VERSION_PATCH	:= 0
endif

WEBRTC_LEAK_DEMO_BUILD_HOST	:=	$(shell hostname)
WEBRTC_LEAK_DEMO_BUILD_TIME	:=	$(shell date '+%a %b %d, %Y %H:%M %Z %z')

################################################################################
# Build/Stage Target Determination
################################################################################

WEBRTC_LEAK_DEMO_BUILD_ARCH_NOT_SUPPORTED		:=	build-arch-not-supported
WEBRTC_LEAK_DEMO_BUILD_PLATFORM_NOT_SUPPORTED	:=	build-platform-not-supported
WEBRTC_LEAK_DEMO_BUILD_TYPE_NOT_SUPPORTED		:=	build-type-not-supported

ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))
WEBRTC_LEAK_DEMO_BUILD_TARGET	:=	build
ifeq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG))
WEBRTC_LEAK_DEMO_STAGE_TARGET	:=	stage-debug
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_RELEASE))
WEBRTC_LEAK_DEMO_STAGE_TARGET	:=	stage-release
endif # ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG))
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_LINUX))
WEBRTC_LEAK_DEMO_BUILD_TARGET	:=	build
ifeq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG))
WEBRTC_LEAK_DEMO_STAGE_TARGET	:=	stage-debug
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_RELEASE))
WEBRTC_LEAK_DEMO_STAGE_TARGET	:=	stage-release
endif # ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG))
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_WINDOWS))
WEBRTC_LEAK_DEMO_BUILD_TARGET	:=	build
ifeq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG))
WEBRTC_LEAK_DEMO_STAGE_TARGET	:=	stage-debug
else ifeq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_RELEASE))
WEBRTC_LEAK_DEMO_STAGE_TARGET	:=	stage-release
endif # ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG))
else # ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))
WEBRTC_LEAK_DEMO_BUILD_TARGET	:=	$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_NOT_SUPPORTED)
endif # ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_FREEBSD))

ifneq ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_NOT_SUPPORTED))
ifneq ($(WEBRTC_LEAK_DEMO_BUILD_ARCH),$(WEBRTC_LEAK_DEMO_BUILD_ARCH_AMD64))
WEBRTC_LEAK_DEMO_BUILD_TARGET	:=	$(WEBRTC_LEAK_DEMO_BUILD_ARCH_NOT_SUPPORTED)
endif # ($(WEBRTC_LEAK_DEMO_BUILD_ARCH),$(WEBRTC_LEAK_DEMO_BUILD_ARCH_AMD64))
endif # ($(WEBRTC_LEAK_DEMO_BUILD_PLATFORM),$(WEBRTC_LEAK_DEMO_BUILD_PLATFORM_NOT_SUPPORTED))

ifneq ($(WEBRTC_LEAK_DEMO_BUILD_ARCH),$(WEBRTC_LEAK_DEMO_BUILD_ARCH_NOT_SUPPORTED))
ifneq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG))
ifneq ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_RELEASE))
WEBRTC_LEAK_DEMO_BUILD_TARGET	:=	$(WEBRTC_LEAK_DEMO_BUILD_TYPE_NOT_SUPPORTED)
endif # ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_RELEASE))
endif # ($(WEBRTC_LEAK_DEMO_BUILD_TYPE),$(WEBRTC_LEAK_DEMO_BUILD_TYPE_DEBUG))
endif # ($(WEBRTC_LEAK_DEMO_BUILD_ARCH),$(WEBRTC_LEAK_DEMO_BUILD_ARCH_NOT_SUPPORTED))

################################################################################
# Make Targets - All
################################################################################

.PHONY: all
all: info
all: install-build-deps
all: diag
all: $(WEBRTC_LEAK_DEMO_BUILD_TARGET)
all: $(WEBRTC_LEAK_DEMO_STAGE_TARGET)

################################################################################
# Make Targets - Misc
################################################################################

.PHONY: info
info:
	@echo "Building $(WEBRTC_LEAK_DEMO_PROJECT_NAME) $(WEBRTC_LEAK_DEMO_VERSION)..."
	@echo "  - Git Branch       :  $(WEBRTC_LEAK_DEMO_GIT_BRANCH)"
	@echo "  - Git Tag          :  $(WEBRTC_LEAK_DEMO_GIT_TAG)"
	@echo "  - Git Commit       :  $(WEBRTC_LEAK_DEMO_GIT_COMMIT)"
	@echo "  - Git Tree State   :  $(WEBRTC_LEAK_DEMO_GIT_TREE_STATE)"
	@echo "  - Version Major    :  $(WEBRTC_LEAK_DEMO_VERSION_MAJOR)"
	@echo "  - Version Minor    :  $(WEBRTC_LEAK_DEMO_VERSION_MINOR)"
	@echo "  - Version Patch    :  $(WEBRTC_LEAK_DEMO_VERSION_PATCH)"
	@echo "  - Build Host       :  $(WEBRTC_LEAK_DEMO_BUILD_HOST)"
	@echo "  - Build Time       :  $(WEBRTC_LEAK_DEMO_BUILD_TIME)"

.PHONY: clean
clean:
	@echo "Cleaning up '$(WEBRTC_LEAK_DEMO_PROJECT_NAME)'..."
	@echo "Removing stage/*..."
	@rm -rf stage
	@echo "Running cargo clean..."
	@cargo clean
	@echo "'$(WEBRTC_LEAK_DEMO_PROJECT_NAME)' has been cleaned up successfully!"

.PHONY: install-build-deps
install-build-deps:
	@echo "Installing build dependencies if they have not already been installed..."
	@cargo install cargo-audit

################################################################################
# Make Targets - Diagnostics
################################################################################

.PHONY: audit
audit:
	@echo "Auditing the dependencies for vulnerabilities..."
	@cargo audit

.PHONY: clippy
clippy:
	@echo "Running the Rust clippy linter..."
	@cargo clippy -- -W clippy::pedantic

.PHONY: fmt-check
fmt-check:
	@echo "Running the Rust formatting checks..."
	@cargo fmt --check

.PHONY: diag
diag: fmt-check
diag: clippy
diag: audit

################################################################################
# Make Targets - Build
################################################################################

.PHONY: build-arch-not-supported
build-arch-not-supported:
	$(error Target CPU architecture is not supported!)

.PHONY: build-platform-not-supported
build-platform-not-supported:
	$(error Target operating system is not supported!)

.PHONY: build-type-not-supported
build-type-not-supported:
	$(error Target build type is not supported!)

.PHONY: build
build: build-webrtc-leak-demo

.PHONY: build-webrtc-leak-demo
build-webrtc-leak-demo:
	@echo "Building '$(WEBRTC_LEAK_DEMO_TARGET_NAME)' in '$(WEBRTC_LEAK_DEMO_BUILD_TYPE)' mode..."
	@cargo build \
		--manifest-path="$(WEBRTC_LEAK_DEMO_MANIFEST_FILE)" \
		--target "$(WEBRTC_LEAK_DEMO_BUILD_TARGET_TRIPPLE)" \
		$(WEBRTC_LEAK_DEMO_BUILD_FLAGS)

################################################################################
# Make Targets - Stage
################################################################################

.PHONY: stage-common
stage-common:
	@echo "Prepating the stage dir '$(WEBRTC_LEAK_DEMO_STAGE_DIR)'..."
	@mkdir -p "$(WEBRTC_LEAK_DEMO_STAGE_DIR)"

.PHONY: stage-debug
stage-debug: stage-common
stage-debug:
	@echo "Staging Linux/Debug binary of '$(WEBRTC_LEAK_DEMO_TARGET_NAME)'..."
	@cp -vr "$(WEBRTC_LEAK_DEMO_TARGET_BIN_DEBUG)" "$(WEBRTC_LEAK_DEMO_STAGE_DIR)"/

.PHONY: stage-release
stage-release: stage-common
stage-release:
	@echo "Staging Linux/Release binary of '$(WEBRTC_LEAK_DEMO_TARGET_NAME)'..."
	@cp -vr "$(WEBRTC_LEAK_DEMO_TARGET_BIN_RELEASE)" "$(WEBRTC_LEAK_DEMO_STAGE_DIR)"/