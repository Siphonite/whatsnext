import React from "react";

interface Tab {
  id: string;
  label: string;
}

interface TabsProps {
  tabs: Tab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
}

const Tabs: React.FC<TabsProps> = ({ tabs, activeTab, onTabChange }) => {
  return (
    <div className="flex gap-2 border-b border-gray-700 mb-6">
      {tabs.map((tab) => {
        const isActive = activeTab === tab.id;
        return (
          <button
            key={tab.id}
            onClick={() => onTabChange(tab.id)}
            className={`
              px-6 py-3 font-semibold text-sm transition-all duration-200 relative
              ${isActive 
                ? "text-cyan-400" 
                : "text-gray-400 hover:text-gray-300"
              }
            `}
          >
            {tab.label}
            {isActive && (
              <div 
                className="absolute bottom-0 left-0 right-0 h-0.5 bg-cyan-400"
                style={{
                  boxShadow: "0 0 10px rgba(6, 182, 212, 0.8)",
                }}
              />
            )}
          </button>
        );
      })}
    </div>
  );
};

export default Tabs;

