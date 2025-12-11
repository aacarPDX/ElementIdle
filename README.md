# ElementIdol

### Description
An idol game where you can collect subatomic particles and unlock elements. You can get upgrades to collect subatomic particles more quickly. 

Unlocking elements can help with collecting subatomic particles, which in turn can help you unlock other elements. The elements are from the Periodic Table of Elements.

### How to build
run the followign command:

cargo run --release

### What was built and how it works


The current build has a button (on the left side of the UI under "Elements") that allows you to increment the number of Electrons (subatomic particle). You are able to purchase three types of upgrades (on the right side of the UI under "Upgrades"). The button ot purcahse the upgrade is uninteractable until you have a sufficient amount of Electrons to purchase the upgrade. Once you have gotten enough, you are able to click the button to purchase the upgrade. Purchasing the upgrade results in the the cost of the upgrade being deducted from your Electrons. Upgrades have different types, which can be for AutoScooperQuantity or Production boost. The upgrade also have effects. These effects can either be additive or multiplicative. What these effects are applied to change according to their types.

### What Didn't Work

Increasing Upgrade tier and upgrade cost. With how I implemented upgrades and the ui, I am not able to change their tiers or their cost. So the upgrades will always have the same price. I also tried implementing this with ggez and egui using a crate called ggegui. I think this wasn't and probably should have done something else.

### Lessons Learned
Clicker games have a lot more to them then I had initially thought. I have been told by someone that implementing a clicker game properly would make you a better programmer. I didn't believe it then, but I do now. 

I need to commit changes more frequently as I have lost some code here and there while programming this.
