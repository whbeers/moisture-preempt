simple moisture sensor using rust w/ cortex-m-rtic running on on stm32f1 "blue pill" board.

WIP, currently a PoC that flashes an LED as an indication of moisture readings, driven by TIM2/TIM3 timer interrupts.

goal is to build this into an efficient, battery-powered garden moisture sensor. details of reading aggregation is TBD, but I already bought a bunch of esp8266 boards so wifi is likely to be involved.
