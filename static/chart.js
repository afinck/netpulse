// filepath: /netpulse/netpulse/static/chart.js
const ctx = document.getElementById('myChart').getContext('2d');
const myChart = new Chart(ctx, {
    type: 'line', // Change this to 'bar', 'pie', etc. as needed
    data: {
        labels: [], // Populate with your data labels
        datasets: [{
            label: 'Measurement Data',
            data: [], // Populate with your measurement data
            borderColor: 'rgba(75, 192, 192, 1)',
            backgroundColor: 'rgba(75, 192, 192, 0.2)',
            borderWidth: 1
        }]
    },
    options: {
        scales: {
            y: {
                beginAtZero: true
            }
        }
    }
});

// Function to fetch data from the backend and update the chart
async function fetchData() {
    const response = await fetch('/api/measurements'); // Adjust the endpoint as necessary
    const data = await response.json();
    
    // Update chart data
    myChart.data.labels = data.labels; // Assuming the response has labels
    myChart.data.datasets[0].data = data.values; // Assuming the response has values
    myChart.update();
}

// Call fetchData on page load
window.onload = fetchData;