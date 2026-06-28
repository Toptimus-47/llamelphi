import requests
import json
import time
import sys

BASE_URL = "http://127.0.0.1:8000"
SESSION_ID = "python-integration-test-01"

def test_pii_masking_and_stream():
    print(">>> [TEST] Starting PII Masking & Stream Integration Test...")
    
    payload = {
        "query": "My name is John Doe (ID: 900101-1234567). Explain Clean Architecture.",
        "session_id": SESSION_ID
    }
    
    try:
        # 1. Test Streaming API
        print(f"Connecting to {BASE_URL}/chat/stream...")
        response = requests.post(f"{BASE_URL}/chat/stream", json=payload, stream=True, timeout=30)
        
        if response.statusCode != 200:
            print(f"[FAIL] Backend returned status code {response.status_code}")
            return False

        full_content = ""
        pii_leaked = False
        
        print("Receiving SSE Stream...")
        for line in response.iter_lines():
            if line:
                decoded_line = line.decode('utf-8')
                if decoded_line.startswith("data: "):
                    data_str = decoded_line[6:]
                    if data_str == "[DONE]":
                        break
                    
                    try:
                        data_json = json.loads(data_str)
                        content = data_json.get("content", "")
                        full_content += content
                        
                        # Real-time leak check
                        if "John Doe" in content or "900101-1234567" in content:
                            pii_leaked = True
                    except json.JSONDecodeError:
                        continue

        print("\n--- Stream Completed ---")
        
        # 2. Verify PII Masking in Output
        if pii_leaked:
            print("[FAIL] PII LEAKED in the stream!")
            return False
        
        # 3. Verify Session History for Masking
        print("Verifying Session History...")
        history_resp = requests.get(f"{BASE_URL}/sessions/{SESSION_ID}/history")
        if history_resp.status_code == 200:
            history = history_resp.json()
            # Convert history to string for searching
            history_str = json.dumps(history)
            
            if "John Doe" in history_str:
                print("[FAIL] PII found in stored history!")
                return False
            if "[NAME_HIDDEN]" in history_str or "[ID_HIDDEN]" in history_str:
                print("[PASS] PII successfully masked in history.")
            else:
                print("[WARN] Masking markers not found, but raw PII is absent. Check masking policy.")
        else:
            print(f"[WARN] Could not fetch history (Status: {history_resp.status_code})")

        print(">>> [SUCCESS] PII Integration Test Passed.")
        return True

    except Exception as e:
        print(f"[ERROR] Test failed due to exception: {e}")
        return False

def test_specialist_routing():
    print("\n>>> [TEST] Starting Specialist Routing Test (Code Analysis)...")
    code_query = "Review this Rust code for safety: \n```rust\nfn main() { unsafe { *std::ptr::null::<i32>() = 1; } }\n```"
    
    payload = {
        "query": code_query,
        "session_id": "routing-test-01"
    }
    
    try:
        response = requests.post(f"{BASE_URL}/chat/stream", json=payload, stream=True, timeout=30)
        # We check if the logs or units in the stream mention Gushnasaph or Specialist
        expert_engaged = False
        
        for line in response.iter_lines():
            if line:
                decoded_line = line.decode('utf-8')
                if "Gushnasaph" in decoded_line or "Specialist" in decoded_line:
                    expert_engaged = True
        
        if expert_engaged:
            print("[PASS] Code Specialist (Gushnasaph) was engaged.")
            return True
        else:
            print("[FAIL] Expert routing did not trigger for code query.")
            return False
    except Exception as e:
        print(f"[ERROR] Routing test failed: {e}")
        return False

def test_web_retrieval():
    print("\n>>> [TEST] Starting Web Retrieval Test (Real-time Telemetry)...")
    # Query that forces web search routing
    web_query = "What is the current market trend of NVIDIA in May 2026? Use web search."
    
    payload = {
        "query": web_query,
        "session_id": "web-test-01"
    }
    
    try:
        response = requests.post(f"{BASE_URL}/chat/stream", json=payload, stream=True, timeout=60)
        
        web_telemetry_active = False
        search_results_found = False
        
        for line in response.iter_lines():
            if line:
                decoded_line = line.decode('utf-8')
                # Check for telemetry events or status messages
                if "Web Telemetry" in decoded_line or "search_strategy" in decoded_line:
                    web_telemetry_active = True
                
                # Check for actual search results markers from web_searcher.rs
                if "[LIVE WEB SEARCH RESULTS]" in decoded_line or "documents" in decoded_line:
                    search_results_found = True
        
        if web_telemetry_active:
            print("[PASS] Web Telemetry phase was activated.")
            if search_results_found:
                print("[PASS] Web search results were successfully retrieved.")
                return True
            else:
                print("[WARN] Web Telemetry started but no search results were captured in stream.")
                print("       (This might happen if TAVILY_API_KEY is missing or query didn't trigger search)")
                return False
        else:
            print("[FAIL] Web search was not triggered for the real-time query.")
            return False
            
    except Exception as e:
        print(f"[ERROR] Web retrieval test failed: {e}")
        return False

if __name__ == "__main__":
    print("====================================================")
    print("   MAGI 2026 LOCAL INTEGRATION TEST SUITE (Python)  ")
    print("   (No Virtualization/Docker Required)              ")
    print("====================================================\n")
    
    # Check if server is up
    try:
        requests.get(BASE_URL, timeout=2)
    except:
        print(f"[CRITICAL] Backend server not found at {BASE_URL}")
        print("Please run 'cargo run' in magi_core before starting this test.")
        sys.exit(1)

    s1 = test_pii_masking_and_stream()
    s2 = test_specialist_routing()
    s3 = test_web_retrieval()
    
    print("\n====================================================")
    if s1 and s2 and s3:
        print("   FINAL VERDICT: [ PASS ]")
    else:
        print("   FINAL VERDICT: [ FAIL ]")
    print("====================================================")
