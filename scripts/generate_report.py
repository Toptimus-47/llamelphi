import json
import os

def generate_html_report(json_path, output_path):
    if not os.path.exists(json_path):
        print(f"Error: {json_path} not found.")
        return

    with open(json_path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    html_content = f"""
    <!DOCTYPE html>
    <html lang="ko">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>MAGI Consensus Process Report</title>
        <style>
            body {{
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                background-color: #09090b;
                color: #e4e4e7;
                line-height: 1.6;
                padding: 20px;
            }}
            .container {{
                max-width: 1200px;
                margin: 0 auto;
            }}
            h1 {{
                color: #10b981;
                border-bottom: 2px solid #10b981;
                padding-bottom: 10px;
                text-align: center;
            }}
            .scenario-card {{
                background-color: #18181b;
                border: 1px solid #27272a;
                border-radius: 8px;
                margin-bottom: 40px;
                padding: 25px;
                box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
            }}
            .query-section {{
                background-color: #27272a;
                padding: 15px;
                border-radius: 6px;
                margin-bottom: 20px;
                font-weight: bold;
                color: #fbbf24;
            }}
            .grid {{
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 20px;
            }}
            .column {{
                background-color: #09090b;
                padding: 15px;
                border-radius: 6px;
                border-left: 4px solid #3f3f46;
            }}
            .column h3 {{
                margin-top: 0;
                color: #3b82f6;
                font-size: 0.9rem;
                text-transform: uppercase;
                letter-spacing: 1px;
            }}
            .initial-draft {{ border-left-color: #6366f1; }}
            .critique-section {{
                grid-column: span 2;
                background-color: #450a0a;
                padding: 15px;
                border-radius: 6px;
                margin: 20px 0;
                border-left: 4px solid #ef4444;
            }}
            .critique-section h3 {{ color: #f87171; margin-top: 0; }}
            .final-consensus {{
                grid-column: span 2;
                background-color: #064e3b;
                padding: 20px;
                border-radius: 6px;
                border-left: 4px solid #10b981;
            }}
            .final-consensus h3 {{ color: #34d399; margin-top: 0; }}
            pre {{
                white-space: pre-wrap;
                word-wrap: break-word;
                font-size: 0.95rem;
            }}
            .badge {{
                display: inline-block;
                padding: 2px 8px;
                border-radius: 4px;
                font-size: 0.8rem;
                font-weight: bold;
                margin-right: 10px;
            }}
            .badge-melchior {{ background-color: #3b82f6; color: white; }}
            .badge-balthasar {{ background-color: #ef4444; color: white; }}
        </style>
    </head>
    <body>
        <div class="container">
            <h1>MAGI: Adversarial Consensus Process Report</h1>
    """

    for entry in data:
        scenario = entry.get('scenario', 'N/A')
        query = entry.get('query', 'N/A')
        initial = entry.get('initial_draft', 'No initial draft recorded.')
        final = entry.get('final_answer', 'No final answer recorded.')
        critiques = entry.get('critiques', [])

        html_content += f"""
            <div class="scenario-card">
                <div class="query-section">
                    Scenario #{scenario}: {query}
                </div>
                <div class="grid">
                    <div class="column initial-draft">
                        <h3><span class="badge badge-melchior">Melchior</span> Initial Draft</h3>
                        <pre>{initial}</pre>
                    </div>
                    <div class="column">
                        <h3>Process Status</h3>
                        <p>Total Revisions: {len(critiques)} cycle(s)</p>
                        <p>Rigor Level: Standard</p>
                    </div>
                </div>
        """

        if critiques:
            html_content += """
                <div class="critique-section">
                    <h3><span class="badge badge-balthasar">Balthasar</span> Adversarial Critiques</h3>
            """
            for critic_name, critique_text in critiques:
                html_content += f"""
                    <div style="margin-bottom: 15px; border-bottom: 1px solid #7f1d1d; padding-bottom: 10px;">
                        <strong>Agent: {critic_name}</strong>
                        <pre>{critique_text}</pre>
                    </div>
                """
            html_content += "</div>"

        html_content += f"""
                <div class="final-consensus">
                    <h3><span class="badge" style="background-color: #10b981;">MAGI</span> Final Consensus Report</h3>
                    <pre>{final}</pre>
                </div>
            </div>
        """

    html_content += """
        </div>
    </body>
    </html>
    """

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(html_content)
    print(f"Successfully generated HTML report: {output_path}")

if __name__ == "__main__":
    generate_html_report('test_data/quality_evaluation_report.json', 'test_data/MAGI_Consensus_Report.html')
