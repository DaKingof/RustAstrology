#!/usr/bin/env node

// Simple test script to verify the Rust Astrology application
const { chromium } = require('playwright');

async function testApplication() {
    let browser;
    try {
        // Launch browser in headless mode
        browser = await chromium.launch({ 
            headless: true,
            args: ['--no-sandbox', '--disable-setuid-sandbox']
        });
        
        const context = await browser.newContext({
            viewport: { width: 1200, height: 800 }
        });
        
        const page = await context.newPage();
        
        console.log('🚀 Navigating to Rust Astrology application...');
        await page.goto('http://127.0.0.1:8083/', { waitUntil: 'networkidle' });
        
        // Wait for the application to load
        await page.waitForTimeout(3000);
        
        console.log('📄 Checking page title...');
        const title = await page.title();
        console.log(`Page title: ${title}`);
        
        console.log('🔍 Checking for key elements...');
        
        // Check for main header
        const header = await page.locator('h1').textContent();
        console.log(`Header: ${header}`);
        
        // Check for dial container
        const dialContainer = await page.locator('.dial-container').count();
        console.log(`Dial containers found: ${dialContainer}`);
        
        // Check for canvas element
        const canvas = await page.locator('canvas.astrology-dial').count();
        console.log(`Canvas elements found: ${canvas}`);
        
        // Check for loading spinner (should be gone by now)
        const loadingSpinner = await page.locator('.loading-spinner').count();
        console.log(`Loading spinners found: ${loadingSpinner}`);
        
        // Check for error displays
        const errorDisplay = await page.locator('.error-display').count();
        console.log(`Error displays found: ${errorDisplay}`);
        
        // Check for dial info
        const dialInfo = await page.locator('.dial-info').count();
        console.log(`Dial info panels found: ${dialInfo}`);
        
        // Check for instructions
        const instructions = await page.locator('.instructions').count();
        console.log(`Instruction panels found: ${instructions}`);
        
        console.log('📸 Taking screenshot...');
        await page.screenshot({ 
            path: '/home/mend/Projects/RustAstrology/test-screenshot.png',
            fullPage: true 
        });
        
        console.log('🎯 Testing canvas functionality...');
        
        // Get canvas dimensions
        const canvasElement = page.locator('canvas.astrology-dial');
        if (await canvasElement.count() > 0) {
            const boundingBox = await canvasElement.boundingBox();
            if (boundingBox) {
                console.log(`Canvas dimensions: ${boundingBox.width}x${boundingBox.height}`);
                console.log(`Canvas position: (${boundingBox.x}, ${boundingBox.y})`);
                
                // Test mouse interaction on canvas
                console.log('🖱️ Testing mouse interaction...');
                await page.mouse.move(boundingBox.x + boundingBox.width/2, boundingBox.y + boundingBox.height/2);
                await page.mouse.down();
                await page.mouse.move(boundingBox.x + boundingBox.width/2 + 50, boundingBox.y + boundingBox.height/2);
                await page.mouse.up();
                
                // Wait a moment for any updates
                await page.waitForTimeout(1000);
            } else {
                console.log('❌ Canvas element found but no bounding box');
            }
        } else {
            console.log('❌ No canvas element found');
        }
        
        console.log('📊 Checking dynamic content...');
        
        // Check if rotation value is displayed and changing
        const rotationText = await page.locator('.dial-info p').filter({ hasText: 'Current Rotation' }).textContent();
        console.log(`Rotation display: ${rotationText || 'Not found'}`);
        
        // Check alignments count
        const alignmentsText = await page.locator('.dial-info p').filter({ hasText: 'Active Alignments' }).textContent();
        console.log(`Alignments display: ${alignmentsText || 'Not found'}`);
        
        console.log('🎨 Checking visual styling...');
        
        // Check if CSS is properly loaded
        const headerColor = await page.locator('h1').evaluate(el => getComputedStyle(el).background);
        console.log(`Header has gradient background: ${headerColor.includes('gradient') ? 'Yes' : 'No'}`);
        
        const dialWrapper = page.locator('.dial-wrapper');
        if (await dialWrapper.count() > 0) {
            const borderRadius = await dialWrapper.evaluate(el => getComputedStyle(el).borderRadius);
            console.log(`Dial wrapper border radius: ${borderRadius}`);
        }
        
        console.log('📱 Testing responsive design...');
        
        // Test mobile viewport
        await page.setViewportSize({ width: 375, height: 667 });
        await page.waitForTimeout(1000);
        
        await page.screenshot({ 
            path: '/home/mend/Projects/RustAstrology/test-screenshot-mobile.png',
            fullPage: true 
        });
        
        console.log('✅ Application testing completed successfully!');
        console.log('📸 Screenshots saved:');
        console.log('  - Desktop: /home/mend/Projects/RustAstrology/test-screenshot.png');
        console.log('  - Mobile: /home/mend/Projects/RustAstrology/test-screenshot-mobile.png');
        
    } catch (error) {
        console.error('❌ Error during testing:', error);
        process.exit(1);
    } finally {
        if (browser) {
            await browser.close();
        }
    }
}

testApplication();
