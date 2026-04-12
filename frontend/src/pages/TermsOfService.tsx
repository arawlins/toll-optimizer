import { 
  Lock, 
  Users, 
} from 'lucide-react';
import { Navbar } from '../components/Navbar';
import { Footer } from '../components/Footer';

/**
 * Toll Optimizer - Terms of Service Screen
 * 
 * This component represents the Terms of Service page for the Toll Optimizer application.
 * It features a clean, editorial layout using Tailwind CSS and the "Equilibrium Finance"
 * design aesthetic, consistent with the Privacy Policy.
 */
export function TermsOfService() {
  return (
    <div className="min-h-screen bg-white font-manrope text-[#1e293b] antialiased">
      {/* Navigation Header */}
      <Navbar />

      <main className="max-w-4xl mx-auto px-6 py-16 md:py-32">
        {/* Header Section */}
        <div className="mb-16 text-center">
          <span className="inline-block px-3 py-1 rounded-full bg-blue-50 text-blue-700 text-[10px] font-bold uppercase tracking-wider mb-6">
            Legal Framework
          </span>
          <h1 className="text-5xl md:text-7xl font-extrabold tracking-tight text-slate-900 mb-6 font-headline">
            Terms of Service
          </h1>
          <p className="text-lg text-slate-500 max-w-2xl mx-auto leading-relaxed">
            Last updated: October 24, 2025. Please read these terms carefully before using the Toll Optimizer platform.
          </p>
        </div>

        {/* Content Sections */}
        <div className="space-y-24">
          
          {/* Section 01: Acceptance */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <div className="flex items-center gap-4">
                <span className="flex items-center justify-center w-8 h-8 rounded-lg bg-slate-100 text-slate-900 text-xs font-black">01</span>
                <h2 className="text-xl font-bold text-slate-900 font-headline">Acceptance of Terms</h2>
              </div>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-6">
                By accessing or using the services provided by Toll Optimizer (the "Service"), you agree to be bound by these Terms of Service. If you do not agree to these terms, please do not use our platform.
              </p>
              <p className="text-slate-600 leading-relaxed">
                Toll Optimizer reserves the right to update and change the Terms of Service from time to time without notice. Any new features that augment or enhance the current Service, including the release of new tools and resources, shall be subject to the Terms of Service.
              </p>
            </div>
          </section>

          {/* Section 02: User Conduct */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <div className="flex items-center gap-4">
                <span className="flex items-center justify-center w-8 h-8 rounded-lg bg-slate-100 text-slate-900 text-xs font-black">02</span>
                <h2 className="text-xl font-bold text-slate-900 font-headline">User Conduct</h2>
              </div>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-6">
                You are responsible for all content posted and activity that occurs under your account. You may not use the Service for any illegal or unauthorized purpose. You must not, in the use of the Service, violate any laws in your jurisdiction.
              </p>
              <ul className="space-y-4">
                <li className="flex gap-3 text-slate-600">
                  <span className="w-1.5 h-1.5 rounded-full bg-blue-600 mt-2 shrink-0"></span>
                  <span>You must not modify, adapt or hack the Service or modify another website so as to falsely imply that it is associated with Toll Optimizer.</span>
                </li>
                <li className="flex gap-3 text-slate-600">
                  <span className="w-1.5 h-1.5 rounded-full bg-blue-600 mt-2 shrink-0"></span>
                  <span>You agree not to reproduce, duplicate, copy, sell, resell or exploit any portion of the Service, use of the Service, or access to the Service without the express written permission by Toll Optimizer.</span>
                </li>
              </ul>
            </div>
          </section>

          {/* Section 03: Account Responsibilities */}
          <section className="bg-slate-50 rounded-[32px] p-8 md:p-12 border border-slate-100">
            <div className="max-w-3xl">
              <div className="flex items-center gap-4 mb-6">
                <span className="flex items-center justify-center w-8 h-8 rounded-lg bg-blue-600 text-white text-xs font-black">03</span>
                <h2 className="text-2xl font-bold text-slate-900 font-headline">Account Responsibilities</h2>
              </div>
              <p className="text-slate-500 leading-relaxed mb-10">
                To access certain features of the platform, you may be required to register for an account. You agree to provide accurate, current, and complete information during the registration process.
              </p>
              
              <div className="grid md:grid-cols-2 gap-6">
                <div className="bg-white rounded-2xl p-8 shadow-sm border border-slate-100">
                  <Lock className="text-blue-600 mb-4 w-6 h-6" />
                  <h3 className="font-bold text-slate-900 text-sm mb-2 uppercase tracking-tight">Security</h3>
                  <p className="text-xs text-slate-500 leading-relaxed">
                    You are responsible for maintaining the confidentiality of your account password.
                  </p>
                </div>
                <div className="bg-white rounded-2xl p-8 shadow-sm border border-slate-100">
                  <Users className="text-blue-600 mb-4 w-6 h-6" />
                  <h3 className="font-bold text-slate-900 text-sm mb-2 uppercase tracking-tight">Ownership</h3>
                  <p className="text-xs text-slate-500 leading-relaxed">
                    Accounts registered by "bots" or other automated methods are not permitted.
                  </p>
                </div>
              </div>
            </div>
          </section>

          {/* Section 04: CSV Data Usage */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <div className="flex items-center gap-4">
                <span className="flex items-center justify-center w-8 h-8 rounded-lg bg-slate-100 text-slate-900 text-xs font-black">04</span>
                <h2 className="text-xl font-bold text-slate-900 font-headline">CSV Data Usage</h2>
              </div>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-8">
                Toll Optimizer allows you to upload CSV files containing toll data for optimization purposes. By uploading this data, you grant Toll Optimizer a worldwide, non-exclusive, royalty-free license to use, copy, and process the data solely for providing the optimization service.
              </p>
              <div className="pl-6 border-l-4 border-orange-400 py-2 bg-orange-50/50 rounded-r-xl">
                <p className="text-sm text-slate-700 italic">
                  We do not sell your uploaded raw CSV data to third parties. We use anonymized aggregate data only to improve our optimization algorithms.
                </p>
              </div>
            </div>
          </section>

          {/* Section 05: Limitation of Liability */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <div className="flex items-center gap-4">
                <span className="flex items-center justify-center w-8 h-8 rounded-lg bg-slate-100 text-slate-900 text-xs font-black">05</span>
                <h2 className="text-xl font-bold text-slate-900 font-headline">Limitation of Liability</h2>
              </div>
            </div>
            <div className="md:col-span-3 text-slate-600 leading-relaxed">
              <p className="mb-6">
                In no event shall Toll Optimizer, nor its directors, employees, partners, agents, suppliers, or affiliates, be liable for any indirect, incidental, special, consequential or punitive damages, including without limitation, loss of profits, data, use, goodwill, or other intangible losses.
              </p>
              <p>
                The Service and all materials are provided "as is" and "as available" without any warranties of any kind, either express or implied, including, but not limited to, implied warranties of merchantability, fitness for a particular purpose, or non-infringement.
              </p>
            </div>
          </section>

          {/* Section 06: Termination */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <div className="flex items-center gap-4">
                <span className="flex items-center justify-center w-8 h-8 rounded-lg bg-slate-100 text-slate-900 text-xs font-black">06</span>
                <h2 className="text-xl font-bold text-slate-900 font-headline">Termination</h2>
              </div>
            </div>
            <div className="md:col-span-3 text-slate-600 leading-relaxed">
              <p className="mb-6">
                We may terminate or suspend access to our Service immediately, without prior notice or liability, for any reason whatsoever, including without limitation if you breach the Terms.
              </p>
              <p>
                Upon termination, your right to use the Service will immediately cease. If you wish to terminate your account, you may simply discontinue using the Service.
              </p>
            </div>
          </section>
        </div>
      </main>

      <Footer />
    </div>
  );
}