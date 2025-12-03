#!/bin/bash

# Awake Test Runner Script
# Runs all tests for the Awake application

set -e

echo "=================================="
echo "  Awake Test Suite Runner"
echo "=================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running in CI mode
CI_MODE=${CI:-false}

# Parse command line arguments
TEST_TYPE="all"
COVERAGE=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --frontend|-f)
      TEST_TYPE="frontend"
      shift
      ;;
    --backend|-b)
      TEST_TYPE="backend"
      shift
      ;;
    --e2e|-e)
      TEST_TYPE="e2e"
      shift
      ;;
    --coverage|-c)
      COVERAGE=true
      shift
      ;;
    --verbose|-v)
      VERBOSE=true
      shift
      ;;
    --help|-h)
      echo "Usage: ./run-tests.sh [OPTIONS]"
      echo ""
      echo "Options:"
      echo "  -f, --frontend    Run only frontend tests"
      echo "  -b, --backend     Run only backend tests"
      echo "  -e, --e2e         Run only end-to-end tests"
      echo "  -c, --coverage    Generate coverage reports"
      echo "  -v, --verbose     Verbose output"
      echo "  -h, --help        Show this help message"
      echo ""
      echo "Examples:"
      echo "  ./run-tests.sh                 # Run all tests"
      echo "  ./run-tests.sh --frontend      # Run only frontend tests"
      echo "  ./run-tests.sh --coverage      # Run all tests with coverage"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

# Test counters
FRONTEND_PASSED=false
BACKEND_PASSED=false
E2E_PASSED=false

# Frontend Tests
run_frontend_tests() {
  echo -e "${YELLOW}[1/2] Running Frontend Tests...${NC}"
  echo "======================================"

  if [ "$COVERAGE" = true ]; then
    if npm run test:coverage; then
      echo -e "${GREEN}✓ Frontend tests passed${NC}"
      FRONTEND_PASSED=true
    else
      echo -e "${RED}✗ Frontend tests failed${NC}"
      return 1
    fi
  else
    if npm run test:run; then
      echo -e "${GREEN}✓ Frontend tests passed${NC}"
      FRONTEND_PASSED=true
    else
      echo -e "${RED}✗ Frontend tests failed${NC}"
      return 1
    fi
  fi
  echo ""
}

# Backend Tests
run_backend_tests() {
  echo -e "${YELLOW}[2/2] Running Backend Tests...${NC}"
  echo "======================================"

  cd src-tauri

  if [ "$VERBOSE" = true ]; then
    if cargo test -- --nocapture; then
      echo -e "${GREEN}✓ Backend tests passed${NC}"
      BACKEND_PASSED=true
    else
      echo -e "${RED}✗ Backend tests failed${NC}"
      cd ..
      return 1
    fi
  else
    if cargo test; then
      echo -e "${GREEN}✓ Backend tests passed${NC}"
      BACKEND_PASSED=true
    else
      echo -e "${RED}✗ Backend tests failed${NC}"
      cd ..
      return 1
    fi
  fi

  cd ..
  echo ""
}

# E2E Tests (standalone only)
run_e2e_tests() {
  echo -e "${YELLOW}Running End-to-End Tests...${NC}"
  echo "======================================"

  if npm test -- e2e.test.ts; then
    echo -e "${GREEN}✓ E2E tests passed${NC}"
    E2E_PASSED=true
  else
    echo -e "${RED}✗ E2E tests failed${NC}"
    return 1
  fi
  echo ""
}

# Check prerequisites
check_prerequisites() {
  echo "Checking prerequisites..."

  # Check Node.js
  if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js is not installed${NC}"
    exit 1
  fi

  # Check npm
  if ! command -v npm &> /dev/null; then
    echo -e "${RED}Error: npm is not installed${NC}"
    exit 1
  fi

  # Check Rust/Cargo
  if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo is not installed${NC}"
    exit 1
  fi

  # Check if node_modules exists
  if [ ! -d "node_modules" ]; then
    echo "Installing npm dependencies..."
    npm install
  fi

  echo -e "${GREEN}✓ Prerequisites check passed${NC}"
  echo ""
}

# Main execution
main() {
  START_TIME=$(date +%s)

  check_prerequisites

  case $TEST_TYPE in
    frontend)
      run_frontend_tests || exit 1
      ;;
    backend)
      run_backend_tests || exit 1
      ;;
    e2e)
      run_e2e_tests || exit 1
      ;;
    all)
      run_frontend_tests || exit 1
      run_backend_tests || exit 1
      ;;
  esac

  END_TIME=$(date +%s)
  DURATION=$((END_TIME - START_TIME))

  echo "======================================"
  echo -e "${GREEN}All tests completed successfully!${NC}"
  echo "======================================"
  echo ""
  echo "Test Results:"

  if [ "$TEST_TYPE" = "all" ] || [ "$TEST_TYPE" = "frontend" ]; then
    [ "$FRONTEND_PASSED" = true ] && echo -e "  Frontend: ${GREEN}✓ PASSED${NC}" || echo -e "  Frontend: ${RED}✗ FAILED${NC}"
  fi

  if [ "$TEST_TYPE" = "all" ] || [ "$TEST_TYPE" = "backend" ]; then
    [ "$BACKEND_PASSED" = true ] && echo -e "  Backend:  ${GREEN}✓ PASSED${NC}" || echo -e "  Backend:  ${RED}✗ FAILED${NC}"
  fi

  if [ "$TEST_TYPE" = "e2e" ]; then
    [ "$E2E_PASSED" = true ] && echo -e "  E2E:      ${GREEN}✓ PASSED${NC}" || echo -e "  E2E:      ${RED}✗ FAILED${NC}"
  fi

  if [ "$TEST_TYPE" = "all" ]; then
    echo "  (E2E tests are included in Frontend tests)"
  fi

  echo ""
  echo "Total execution time: ${DURATION}s"

  if [ "$COVERAGE" = true ]; then
    echo ""
    echo "Coverage reports generated:"
    echo "  - Frontend: coverage/index.html"
    if command -v cargo-tarpaulin &> /dev/null; then
      echo "  - Backend: Run 'cd src-tauri && cargo tarpaulin --out Html' for backend coverage"
    fi
  fi

  echo ""
  echo "For detailed documentation, see TEST_HARNESS.md"
}

# Run main function
main

exit 0
