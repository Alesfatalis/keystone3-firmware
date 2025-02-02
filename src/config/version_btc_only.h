#define CONCAT(x, y) x ## y
#define PLUS10(x) CONCAT(1, x)

#define SOFTWARE_VERSION_MAJOR              1
#define SOFTWARE_VERSION_MAJOR_OFFSET       10
#define SOFTWARE_VERSION_MINOR              1
#define SOFTWARE_VERSION_BUILD              0
#define SOFTWARE_VERSION                    (SOFTWARE_VERSION_MAJOR * 10000 + SOFTWARE_VERSION_MINOR * 100 + SOFTWARE_VERSION_BUILD)

#define SOFTWARE_VERSION_BTC_ONLY_MAJOR     PLUS10(SOFTWARE_VERSION_MAJOR)

#define SOFTWARE_VERSION_SUFFIX             " - BTC"