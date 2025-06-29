<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="/static/stylesheet.css">
    <title>Task Manager</title>

</head>

<style>:root {
    --bg: #1e1e1e;
    --surface: #252526;
    --text: #d4d4d4;
    --text-soft: #858585;
    --accent: #569cd6;
    --highlight: #ce9178;
    --border: #3a3a3a;
}
body {
    background-color: var(--bg);
    color: var(--text);
    font-family: 'Fira Code', monospace;
    line-height: 1.6;
    margin: 0;
    padding: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
}
.container {
    background-color: var(--surface);
    padding: 2rem;
    border-radius: 4px;
    max-width: 600px;
    width: 100%;
    text-align: center;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
}
h1 {
    color: var(--accent);
    font-weight: normal;
    margin-top: 0;
}
.links {
    margin-top: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}
a {
    color: var(--accent);
    text-decoration: none;
    transition: opacity 0.2s;
}
a:hover {
    opacity: 0.8;
    text-decoration: underline;
}
.footer {
    margin-top: 2rem;
    color: var(--text-soft);
    font-size: 0.9rem;
}
</style>
<body>
<div class="container">
    <h1>Task Manager</h1>
    <p>A minimal task manager inspired</p>
    <div class="footer">
        Powered by Rust + Axum
    </div>
</div>
    
</body>
</html>