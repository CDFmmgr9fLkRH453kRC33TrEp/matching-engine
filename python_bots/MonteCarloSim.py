import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt
#from py_vollib_vectorized import vectorized_implied_volatility as implied_vol

file = open(r"SimulatedPriceData.txt", "a")


N = 252                # number of time steps in simulation
M = 6                  # number of simulations
S0 = 100               # Initial price
T = 10                 # Time (years)
kappa = 0.5            # rate of mean reversion of variance under risk-neutral dynamics
theta = 0.60**2        # long-term mean of variance under risk-neutral dynamics
v0 = 0.25**2           # initial variance under risk-neutral dynamics
rho = 0.8              # correlation between returns and variances under risk-neutral dynamics
sigma = 0.4            # volatility of volatility
r = 0.04               # risk-free rate


def heston_model_sim(S0, v0, rho, kappa, theta, sigma, T, N, M):
    # initialise other parameters
    dt = T/N
    mu = np.array([0,0])
    cov = np.array([[1,rho],
                    [rho,1]])

    # arrays for storing prices and variances
    S = np.full(shape=(N+1,M), fill_value=S0)
    v = np.full(shape=(N+1,M), fill_value=v0)

    # sampling correlated brownian motions under risk-neutral measure
    Z = np.random.multivariate_normal(mu, cov, (N,M))

    for i in range(1,N+1):
        S[i] = S[i-1] * np.exp( (r - 0.5*v[i-1])*dt + np.sqrt(v[i-1] * dt) * Z[i-1,:,0] )
        v[i] = np.maximum(v[i-1] + kappa*(theta-v[i-1])*dt + sigma*np.sqrt(v[i-1]*dt)*Z[i-1,:,1],0)
    
    return S, v

S_p,v_p = heston_model_sim(S0, v0, rho, kappa, theta, sigma,T, N, M)

fig, (ax1)  = plt.subplots(1, figsize=(12,5))
time = np.linspace(0,T,N+1)
ax1.plot(time,S_p)
ax1.set_title('Heston Model Asset Prices')
ax1.set_xlabel('Time')
ax1.set_ylabel('Asset Prices')
plt.show()

np.savetxt(file, S_p, delimiter=',')
file.close