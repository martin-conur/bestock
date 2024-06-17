# BeStock 🚧 WIP
### Fast Stock Forecasting
A tool for fast, secure and reliable stock forecasting. 

BeStock is written in Rust, with 4 principles in mind:
* Fast (really fast)
* Safe
* Reliable
* Consistent

## first roadmap: can't be basic than this.
- [x] Naive demand forecast (mean sales + 1 standard deviation)
    - [x] Manage out of stock days, when sales above 0. If 0, just ignore or replace by the mean.
- [ ] Create a SQL database with the sales movements and suggested stock
- [ ] A process that takes all the sales info and stores the suggested stock in the SQL database
- [ ] A GUI for user interaction with suggested stock


