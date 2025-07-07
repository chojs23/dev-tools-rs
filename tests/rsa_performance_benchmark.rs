use dev_tools_rs::core::crypto::asymmetric::rsa::generate_rsa_keypair;
use dev_tools_rs::core::crypto::{CryptographyProcessor, RsaKeySize};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn benchmark_rsa_key_generation() {
    let key_sizes = [
        (RsaKeySize::Rsa512, 512),
        (RsaKeySize::Rsa1024, 1024),
        (RsaKeySize::Rsa2048, 2048),
        (RsaKeySize::Rsa3072, 3072),
        (RsaKeySize::Rsa4096, 4096),
    ];

    println!("RSA Key Generation Performance Benchmark");
    println!("{}", "=".repeat(50));

    for (key_size_enum, bits) in key_sizes.iter() {
        println!("\nTesting RSA-{} bits:", bits);

        let mut times = Vec::new();
        let iterations = if *bits <= 1024 { 5 } else { 3 }; // Fewer iterations for larger keys

        for i in 1..=iterations {
            print!("  Iteration {}/{}: ", i, iterations);

            let start = Instant::now();
            let result = generate_rsa_keypair(*bits);
            let duration = start.elapsed();

            match result {
                Ok((public_key, private_key)) => {
                    println!(
                        "{:.2}s (public: {}..., private: {}...)",
                        duration.as_secs_f64(),
                        &public_key[..50],
                        &private_key[..50]
                    );
                    times.push(duration.as_secs_f64());
                }
                Err(e) => {
                    println!("FAILED: {}", e);
                    break;
                }
            }
        }

        if !times.is_empty() {
            let avg_time = times.iter().sum::<f64>() / times.len() as f64;
            let min_time = times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max_time = times.iter().fold(0.0_f64, |a, &b| a.max(b));

            println!(
                "  Summary: avg={:.2}s, min={:.2}s, max={:.2}s",
                avg_time, min_time, max_time
            );
        }
    }

    println!("\nPerformance Analysis:");
    println!("- RSA-512: Fast but insecure (deprecated)");
    println!("- RSA-1024: Moderate but weak security");
    println!("- RSA-2048: Good balance (current standard)");
    println!("- RSA-3072: Slow but future-proof");
    println!("- RSA-4096: Very slow, overkill for most uses");
}

#[test]
fn test_async_key_generation_performance() {
    use dev_tools_rs::core::crypto::CryptographyProcessor;
    use std::thread;
    use std::time::Duration;

    println!("\nTesting Async RSA Key Generation");
    println!("{}", "=".repeat(40));

    let mut processor = CryptographyProcessor::new();
    processor.input.rsa_key_size = Some(RsaKeySize::Rsa2048);

    println!("Starting async key generation...");
    let start = Instant::now();

    processor
        .start_async_key_generation()
        .expect("Failed to start key generation");

    // Check progress periodically
    let mut checks = 0;
    while !processor.is_key_generation_complete() {
        thread::sleep(Duration::from_millis(100));
        processor.check_key_generation_progress();
        checks += 1;

        if checks % 10 == 0 {
            println!(
                "  Still generating... ({:.1}s)",
                start.elapsed().as_secs_f64()
            );
        }

        // Safety timeout
        if start.elapsed().as_secs() > 120 {
            println!("  Timeout after 2 minutes");
            break;
        }
    }

    let total_time = start.elapsed();
    println!(
        "Async generation completed in {:.2}s",
        total_time.as_secs_f64()
    );

    // Check if keys were generated
    if processor.input.public_key.is_some() && processor.input.private_key.is_some() {
        println!("✓ Keys generated successfully");
    } else {
        println!("✗ Key generation failed");
    }
}

#[test]
#[ignore] // Only run manually due to long execution time
fn comprehensive_performance_test() {
    println!("Comprehensive RSA Performance Test");
    println!("This test measures performance across different scenarios");

    // Test 1: Single-threaded vs multi-threaded
    println!("\n1. Testing threading overhead:");

    // Single-threaded
    let start = Instant::now();
    let _result = generate_rsa_keypair(2048);
    let single_thread_time = start.elapsed();

    // Multi-threaded (using the async approach)
    let mut processor = CryptographyProcessor::new();
    processor.input.rsa_key_size = Some(RsaKeySize::Rsa2048);

    let start = Instant::now();
    processor.start_async_key_generation().unwrap();
    while !processor.is_key_generation_complete() {
        processor.check_key_generation_progress();
        thread::sleep(Duration::from_millis(10));
    }
    let multi_thread_time = start.elapsed();

    println!(
        "  Single-threaded: {:.2}s",
        single_thread_time.as_secs_f64()
    );
    println!("  Multi-threaded:  {:.2}s", multi_thread_time.as_secs_f64());
    println!(
        "  Overhead: {:.2}s",
        (multi_thread_time.as_secs_f64() - single_thread_time.as_secs_f64()).abs()
    );
}
