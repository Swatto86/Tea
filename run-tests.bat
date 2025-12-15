@echo off
REM Tea Test Runner Script for Windows
REM Runs all tests for the Tea application

setlocal enabledelayedexpansion

echo ==================================
echo   Tea Test Suite Runner
echo ==================================
echo.

REM Parse command line arguments
set TEST_TYPE=all
set COVERAGE=false
set VERBOSE=false

:parse_args
if "%~1"=="" goto :end_parse_args
if /i "%~1"=="--frontend" set TEST_TYPE=frontend
if /i "%~1"=="-f" set TEST_TYPE=frontend
if /i "%~1"=="--backend" set TEST_TYPE=backend
if /i "%~1"=="-b" set TEST_TYPE=backend
if /i "%~1"=="--e2e" set TEST_TYPE=e2e
if /i "%~1"=="-e" set TEST_TYPE=e2e
if /i "%~1"=="--coverage" set COVERAGE=true
if /i "%~1"=="-c" set COVERAGE=true
if /i "%~1"=="--verbose" set VERBOSE=true
if /i "%~1"=="-v" set VERBOSE=true
if /i "%~1"=="--help" goto :show_help
if /i "%~1"=="-h" goto :show_help
shift
goto :parse_args

:show_help
echo Usage: run-tests.bat [OPTIONS]
echo.
echo Options:
echo   -f, --frontend    Run only frontend tests
echo   -b, --backend     Run only backend tests
echo   -e, --e2e         Run only end-to-end tests
echo   -c, --coverage    Generate coverage reports
echo   -v, --verbose     Verbose output
echo   -h, --help        Show this help message
echo.
echo Examples:
echo   run-tests.bat                 # Run all tests
echo   run-tests.bat --frontend      # Run only frontend tests
echo   run-tests.bat --coverage      # Run all tests with coverage
exit /b 0

:end_parse_args

REM Test status flags
set FRONTEND_PASSED=false
set BACKEND_PASSED=false
set E2E_PASSED=false
set TESTS_FAILED=false

REM Check prerequisites
echo Checking prerequisites...

where node >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Node.js is not installed
    exit /b 1
)

where npm >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] npm is not installed
    exit /b 1
)

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Cargo is not installed
    exit /b 1
)

if not exist "node_modules\" (
    echo Installing npm dependencies...
    call npm install
    if %errorlevel% neq 0 (
        echo [ERROR] Failed to install dependencies
        exit /b 1
    )
)

echo [OK] Prerequisites check passed
echo.

REM Record start time
set START_TIME=%time%

REM Run tests based on TEST_TYPE
if "%TEST_TYPE%"=="frontend" goto :run_frontend
if "%TEST_TYPE%"=="backend" goto :run_backend
if "%TEST_TYPE%"=="e2e" goto :run_e2e
if "%TEST_TYPE%"=="all" goto :run_all

:run_all
call :run_frontend_tests
if %errorlevel% neq 0 set TESTS_FAILED=true
call :run_backend_tests
if %errorlevel% neq 0 set TESTS_FAILED=true
goto :summary

:run_frontend
call :run_frontend_tests
if %errorlevel% neq 0 set TESTS_FAILED=true
goto :summary

:run_backend
call :run_backend_tests
if %errorlevel% neq 0 set TESTS_FAILED=true
goto :summary

:run_e2e
call :run_e2e_tests
if %errorlevel% neq 0 set TESTS_FAILED=true
goto :summary

REM Frontend Tests Function
:run_frontend_tests
echo [1/2] Running Frontend Tests...
echo ======================================

if "%COVERAGE%"=="true" (
    call npm run test:coverage
) else (
    call npm run test:run
)

if %errorlevel% equ 0 (
    echo [OK] Frontend tests passed
    set FRONTEND_PASSED=true
    echo.
    exit /b 0
) else (
    echo [FAILED] Frontend tests failed
    echo.
    exit /b 1
)

REM Backend Tests Function
:run_backend_tests
echo [2/2] Running Backend Tests...
echo ======================================

cd src-tauri

if "%VERBOSE%"=="true" (
    cargo test -- --nocapture
) else (
    cargo test
)

if %errorlevel% equ 0 (
    echo [OK] Backend tests passed
    set BACKEND_PASSED=true
    cd ..
    echo.
    exit /b 0
) else (
    echo [FAILED] Backend tests failed
    cd ..
    echo.
    exit /b 1
)

REM E2E Tests Function (standalone only)
:run_e2e_tests
echo Running End-to-End Tests...
echo ======================================

call npm run test:run tests/e2e.test.ts

if %errorlevel% equ 0 (
    echo [OK] E2E tests passed
    set E2E_PASSED=true
    echo.
    exit /b 0
) else (
    echo [FAILED] E2E tests failed
    echo.
    exit /b 1
)

:summary
echo ======================================
if "%TESTS_FAILED%"=="true" (
    echo [FAILED] Some tests failed!
) else (
    echo [OK] All tests completed successfully!
)
echo ======================================
echo.
echo Test Results:

if "%TEST_TYPE%"=="all" (
    if "%FRONTEND_PASSED%"=="true" (
        echo   Frontend: [OK] PASSED
    ) else (
        echo   Frontend: [FAILED] FAILED
    )
    if "%BACKEND_PASSED%"=="true" (
        echo   Backend:  [OK] PASSED
    ) else (
        echo   Backend:  [FAILED] FAILED
    )
    REM E2E tests are included in Frontend tests
)

if "%TEST_TYPE%"=="frontend" (
    if "%FRONTEND_PASSED%"=="true" (
        echo   Frontend: [OK] PASSED
    ) else (
        echo   Frontend: [FAILED] FAILED
    )
)

if "%TEST_TYPE%"=="backend" (
    if "%BACKEND_PASSED%"=="true" (
        echo   Backend:  [OK] PASSED
    ) else (
        echo   Backend:  [FAILED] FAILED
    )
)

if "%TEST_TYPE%"=="e2e" (
    if "%E2E_PASSED%"=="true" (
        echo   E2E:      [OK] PASSED
    ) else (
        echo   E2E:      [FAILED] FAILED
    )
)

echo.
set END_TIME=%time%
echo Start time: %START_TIME%
echo End time:   %END_TIME%

if "%COVERAGE%"=="true" (
    echo.
    echo Coverage reports generated:
    echo   - Frontend: coverage\index.html
    echo   - Backend: Run 'cd src-tauri && cargo tarpaulin --out Html' for backend coverage
)

if "%TESTS_FAILED%"=="true" (
    exit /b 1
) else (
    exit /b 0
)
