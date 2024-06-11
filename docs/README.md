# Questions this project aims to answer
## Descriptive Questions

1.  **General Statistics:**
    
    -   What is the total number of spots recorded in a given time period?
    -   How many unique receivers and transmitters are there in the data?
2.  **Time-Based Analysis:**
    
    -   What is the distribution of spots over different times of the day or days of the week?
    -   How does the number of spots vary over different months?
3.  **Geographical Analysis:**
    
    -   What are the most common receiver and transmitter locations (latitude and longitude)?
    -   What is the average distance between transmitters and receivers?

### Analytical Questions

1.  **Signal Propagation:**
    
    -   How does the signal-to-noise ratio (SNR) vary with distance?
    -   What is the relationship between power and distance?
2.  **Frequency and Band Analysis:**
    
    -   How does the frequency distribution look like across different bands?
    -   Which band has the highest average SNR?
3.  **Performance Analysis:**
    
    -   Which version of the software (Version) shows the best performance in terms of SNR?

### Comparative Questions

1.  **Temporal Comparison:**
    
    -   How does the signal propagation vary between different times of the day (e.g., day vs. night)?
    -   How does the performance vary across different months?
2.  **Geographical Comparison:**
    
    -   How does the SNR differ between different geographical regions?
    -   How does the distance vary between different receiver and transmitter locations?

### Complex Analysis

1.  **Pattern Recognition:**
    
    -   Can we identify any patterns in the data that suggest certain times or conditions are better for signal transmission?
    -   Are there any anomalies or outliers in the data that require further investigation?
2.  **Predictive Analysis:**
    
    -   Can we predict the SNR based on other variables such as distance, power, frequency, and band?
    -   Can we build a model to predict the best times and conditions for signal transmission?

### Example Queries

1.  **Basic Query:**
    
    -   How many spots were recorded on the 3rd band on June 10, 2024?
    
```sql
SELECT COUNT(*) 
FROM wspr_data 
WHERE Band = 3 AND Date(Time) = '2024-06-10';
``` 
    
2.  **Signal-to-Noise Ratio Analysis:**
    
    -   What is the average SNR for each band?
    
```sql
SELECT Band, AVG(SNR) 
FROM wspr_data 
GROUP BY Band;
``` 
    
3.  **Distance and SNR Relationship:**
    
    -   What is the correlation between Distance and SNR?
    
```sql
SELECT CORR(Distance, SNR) 
FROM wspr_data;
``` 
    
4.  **Geographical Distribution:**
    
    -   What are the top 5 transmitter locations by the number of spots?
    
```sql
SELECT TX_Loc, COUNT(*) as SpotCount 
FROM wspr_data 
GROUP BY TX_Loc 
ORDER BY SpotCount DESC 
LIMIT 5;
``` 