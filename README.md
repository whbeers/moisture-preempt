simple moisture sensor using rust w/ cortex-m-rtic running on on stm32f1 "blue pill" board.

WIP, currently a PoC that flashes an LED as an indication of moisture readings, driven by TIM2/TIM3 timer interrupts.

goal is to build this into an efficient, battery-powered garden moisture sensor. details of reading aggregation is TBD, but I already bought a bunch of esp8266 boards so wifi is likely to be involved.


current BOM:
 STM32 "blue pill" board: $5 (ebay)
 3.3V regulator: $5 (pololu S7V8F3)
 esp8266 module: $4 (https://www.amazon.com/dp/B01N98BTRH)
 moisture sensor: $7 (https://www.sparkfun.com/products/13637) - degradation is supposedly a problem, the gold-coated contacts might help?
 1Ah LiPo battery: $10 (https://www.sparkfun.com/products/13813)
 JST pigtails to ensure battery remains removable: < $1 (https://www.amazon.com/dp/B07N2LYY7Q/)
 enclosure: $7 (https://www.sparkfun.com/products/16734) - wanted it to be wifi transparent and reasonably weather resistant, should be comfortably-sized for what I'm putting in it
