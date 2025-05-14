## 🤝 Cooperative Strategies

### [X] Always Cooperate (All-C)
Always chooses to cooperate, regardless of opponent behavior.  
Simple, but easily exploited.

### [X] Tit for Tat (TFT)
Starts by cooperating, then mimics the opponent’s last move.  
Forgiving and retaliatory; one of the most successful simple strategies.

### [X] Tit for Two Tats
Cooperates unless the opponent defects two times in a row.  
More forgiving than TFT, tolerates occasional mistakes.

### [X] Generous Tit for Tat
Like TFT, but sometimes cooperates even after a defection (with a small probability).  
Helps prevent echoing retaliation in noisy environments.

### [X] Win-Stay, Lose-Shift (a.k.a. Pavlov)
Cooperate if the previous outcome was rewarding (mutual cooperation or sucker’s payoff); switch otherwise.  
Adaptable and can outperform TFT in some settings.

---

## 😈 Defective / Exploitative Strategies

### [X] Always Defect (All-D)
Always defects, aiming for maximum gain.  
Short-term gains, but performs poorly in the long run.

### [X] Grim Trigger
Cooperates until the opponent defects once, then defects forever.  
Very unforgiving; effective as a deterrent.

### [X] Hard Tit for Tat
Like TFT, but starts by defecting.  
More aggressive than standard TFT.

### [X] Suspicious Tit for Tat
Defects first, then mimics the opponent’s previous move.  
Tests the opponent’s response to defection.

---

## 🔄 Probabilistic or Random Strategies

### [X] Random
Cooperates or defects with equal probability.  
Unpredictable; generally performs poorly.

### [X] Soft Majority
Cooperates if the opponent has cooperated more than half the time.  
Encourages long-term cooperation but is not strict.

### [X] Firm Majority
Defects unless the opponent cooperates more than 75% of the time.  
Stricter threshold for cooperation.

### [X] Stochastic Tit for Tat
Like TFT, but sometimes defects even after cooperation, with some probability.  
Used to test robustness against random noise.

---

## 🧠 Adaptive or Complex Strategies

### [X] Looker Up
Makes decisions based on a lookup table of the last few moves (yours and the opponent’s).  
Capable of intricate behavior depending on implementation.

### Finite State Machine (FSM)
Uses internal states to track history and respond conditionally.  
Allows nuanced memory-based strategies.

### [X] Forgiving
Defects after the opponent defects, but returns to cooperation after a few rounds.  
Encourages reconciliation.

---
[Axelrod's Phyton](https://github.com/Axelrod-Python/Axelrod)