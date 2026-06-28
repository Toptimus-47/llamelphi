import requests
import json
import time
import sys

def run_security_audit_test():
    base_url = "http://127.0.0.1:8000"
    query = (
        "My name is John Doe (Employee ID: EX-2026-991). I am currently evaluating "
        "the firewall logs for Project Aegis at the Seoul Data Center. Based on our "
        "internal security guidelines and the latest NIST Zero Trust framework, can you "
        "analyze why we are seeing a spike in SSH attempts from IP 192.168.1.50? "
        "Also, does this violate our 'Local-First' data privacy policy?"
    )
    
    payload = {
        "query": query,
        "session_id": "scenario-test-security-audit",
        "rigor": "Standard"
    }
    
    print(f"--- Starting Scenario: Project Aegis Security Audit ---")
    print(f"Target URL: {base_url}/chat/stream")
    print(f"Input: {query[:100]}...")
    
    try:
        # We use stream=True because MAGI uses SSE
        response = requests.post(f"{base_url}/chat/stream", json=payload, stream=True, timeout=300)
        
        full_content = ""
        print("Receiving stream: ", end="", flush=True)
        
        for line in response.iter_lines():
            if line:
                decoded_line = line.decode('utf-8')
                if decoded_line.startswith("data: "):
                    data_str = decoded_line[6:]
                    try:
                        data_json = json.loads(data_str)
                        chunk = data_json.get("content", "")
                        full_content += chunk
                        print(".", end="", flush=True)
                    except json.JSONDecodeError:
                        # Might be a simple message like [DONE]
                        pass
        
        print("\n--- Test Analysis ---")
        
        # Validation 1: PII Masking
        pii_leaked = "John Doe" in full_content or "EX-2026-991" in full_content
        print(f"[PII Check] Masking successful: {not pii_leaked}")
        
        # Validation 2: Technical Context
        tech_terms = ["SSH", "Zero Trust", "Firewall"]
        tech_ok = any(term.lower() in full_content.lower() for term in tech_terms)
        print(f"[Tech Check] Security context detected: {tech_ok}")
        
        # Validation 3: Policy Context
        policy_terms = ["Local-First", "Privacy", "Compliance"]
        policy_ok = any(term.lower() in full_content.lower() for term in policy_terms)
        print(f"[Policy Check] Compliance context detected: {policy_ok}")
        
        if not pii_leaked and tech_ok and policy_ok:
            print("\nRESULT: SCENARIO SUCCESS")
        else:
            print("\nRESULT: SCENARIO FAILURE")
            if pii_leaked: print(" - Reason: PII Leakage detected.")
            if not tech_ok: print(" - Reason: Missing technical analysis.")
            if not policy_ok: print(" - Reason: Missing policy analysis.")
            
    except Exception as e:
        print(f"\nError connecting to MAGI Server: {e}")
        print("Ensure the server is running with 'cargo run' before executing this runner.")

if __name__ == "__main__":
    run_security_audit_test()
